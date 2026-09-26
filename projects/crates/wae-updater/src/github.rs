use crate::error::{Result, UpdateError};
use serde::Deserialize;

const API_BASE: &str = "https://api.github.com";

#[derive(Debug, Clone)]
pub struct GitHubReleaseSource {
    pub owner: String,
    pub repo: String,
}

impl GitHubReleaseSource {
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn from_slug(slug: &str) -> Result<Self> {
        let parts = slug.split('/').filter(|p| !p.is_empty()).collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(UpdateError::InvalidRepo(slug.to_string()));
        }
        Ok(Self::new(parts[0], parts[1]))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub html_url: String,
    pub draft: bool,
    pub prerelease: bool,
    pub assets: Vec<GitHubReleaseAsset>,
}

pub(crate) struct GitHubClient {
    source: GitHubReleaseSource,
    token: Option<String>,
}

impl GitHubClient {
    pub fn new(source: GitHubReleaseSource, token: Option<String>) -> Self {
        Self { source, token }
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            "application/vnd.github+json".parse().expect("accept"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            "2022-11-28".parse().expect("api version"),
        );
        headers.insert(reqwest::header::USER_AGENT, "wae-updater".parse().expect("ua"));
        if let Some(token) = &self.token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").parse().expect("auth"),
            );
        }
        headers
    }

    pub fn fetch_latest(&self) -> Result<GitHubRelease> {
        let url = format!(
            "{}/repos/{}/{}/releases/latest",
            API_BASE,
            self.source.owner,
            self.source.repo
        );
        let release = self.get_json(&url)?;
        if release.draft {
            return Err(UpdateError::Other(format!(
                "Latest release {} is a draft",
                release.tag_name
            )));
        }
        Ok(release)
    }

    pub fn fetch_tag(&self, tag: &str) -> Result<GitHubRelease> {
        let url = format!(
            "{}/repos/{}/{}/releases/tags/{}",
            API_BASE,
            self.source.owner,
            self.source.repo,
            urlencoding_path(tag)
        );
        self.get_json(&url)
    }

    pub fn download_asset(&self, asset: &GitHubReleaseAsset, dest: &std::path::Path) -> Result<()> {
        let mut headers = self.headers();
        headers.insert(
            reqwest::header::ACCEPT,
            "application/octet-stream".parse().expect("accept"),
        );
        let client = reqwest::blocking::Client::new();
        let mut response = client
            .get(&asset.browser_download_url)
            .headers(headers)
            .send()?;
        if !response.status().is_success() {
            return Err(UpdateError::Other(format!(
                "Download failed ({}) for {}",
                response.status(),
                asset.name
            )));
        }
        let mut file = std::fs::File::create(dest).map_err(|e| UpdateError::io(dest, e))?;
        std::io::copy(&mut response, &mut file).map_err(|e| UpdateError::io(dest, e))?;
        Ok(())
    }

    fn get_json(&self, url: &str) -> Result<GitHubRelease> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).headers(self.headers()).send()?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().unwrap_or_default();
            return Err(UpdateError::Other(format!("GitHub API {status}: {body}")));
        }
        Ok(response.json()?)
    }
}

fn urlencoding_path(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c.to_string()
            } else {
                format!("%{:02X}", c as u8)
            }
        })
        .collect()
}

/// Pick a GitHub Release archive for a **`wae build` product** on this target triple.
pub fn pick_product_asset<'a>(
    product_name: &str,
    triple: &str,
    os_arch: &str,
    assets: &'a [GitHubReleaseAsset],
) -> Option<&'a GitHubReleaseAsset> {
    let candidates = [
        format!("{product_name}-{triple}.zip"),
        format!("{product_name}-{triple}.tar.gz"),
        format!("{product_name}-{triple}.tgz"),
        format!("{product_name}-native-{triple}.zip"),
        format!("{product_name}-{os_arch}.zip"),
        format!("{product_name}-{os_arch}.tar.gz"),
        format!("{product_name}-{triple}"),
    ];
    for name in &candidates {
        if let Some(asset) = assets.iter().find(|a| a.name == *name) {
            return Some(asset);
        }
    }
    assets
        .iter()
        .find(|a| a.name.contains(product_name) && a.name.contains(triple))
}
