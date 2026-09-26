//! GitHub Releases self-update for **`wae build` products** (shipped apps).
//!
//! WAE (`@wae/wae`) is a dev toolchain — upgrade it with npm. End-user updates target the
//! **built product**: frontend bundle + native host addon (`wae-napi` `.node` / platform dylib).

#![warn(missing_docs)]

mod apply;
mod archive;
mod error;
mod github;
mod target;

pub use error::{Result, UpdateError};
pub use github::{GitHubRelease, GitHubReleaseAsset, GitHubReleaseSource, pick_product_asset};
pub use target::{host_os_arch, host_triple};

use std::path::PathBuf;

use semver::Version;

use crate::apply::{replace_native_artifact, stage_release_asset};
use crate::github::GitHubClient;

/// Configuration for a product self-update check/apply cycle.
#[derive(Debug, Clone)]
pub struct UpdateOptions {
    /// GitHub Releases source — typically **your app's** `owner/repo`, not the WAE framework repo.
    pub source: GitHubReleaseSource,
    pub current_version: Version,
    /// Product slug used in release asset names (`{product_name}-{triple}.zip`).
    pub product_name: String,
    /// Installed native addon path (e.g. `lib/win32-x64-msvc.node` inside the built app tree).
    pub native_path: PathBuf,
    /// Defaults to [`host_triple`].
    pub target_triple: Option<String>,
    pub token: Option<String>,
    pub allow_prerelease: bool,
    /// Pin a release tag instead of `/releases/latest`.
    pub tag: Option<String>,
}

/// Describes an available newer release.
#[derive(Debug, Clone)]
pub struct UpdateAvailability {
    pub current: Version,
    pub latest: Version,
    pub tag: String,
    pub release_url: String,
    pub asset: GitHubReleaseAsset,
}

pub struct SelfUpdate {
    options: UpdateOptions,
}

impl SelfUpdate {
    pub fn new(options: UpdateOptions) -> Self {
        Self { options }
    }

    fn client(&self) -> GitHubClient {
        let token = self
            .options
            .token
            .clone()
            .or_else(|| std::env::var("WAE_GITHUB_TOKEN").ok())
            .or_else(|| std::env::var("GITHUB_TOKEN").ok());
        GitHubClient::new(self.options.source.clone(), token)
    }

    fn triple(&self) -> String {
        self.options
            .target_triple
            .clone()
            .unwrap_or_else(|| host_triple().to_string())
    }

    fn resolve_release(&self, client: &GitHubClient) -> Result<GitHubRelease> {
        let release = if let Some(tag) = &self.options.tag {
            client.fetch_tag(tag)?
        } else {
            client.fetch_latest()?
        };
        if release.prerelease && !self.options.allow_prerelease && self.options.tag.is_none() {
            return Err(UpdateError::Other(format!(
                "Latest release {} is prerelease; set allow_prerelease or pin --tag",
                release.tag_name
            )));
        }
        Ok(release)
    }

    /// Returns `None` when already on or ahead of the resolved release.
    pub fn check(&self) -> Result<Option<UpdateAvailability>> {
        let client = self.client();
        let release = self.resolve_release(&client)?;
        let latest = normalize_release_tag(&release.tag_name)?;
        if self.options.current_version >= latest {
            return Ok(None);
        }
        let triple = self.triple();
        let os_arch = host_os_arch();
        let asset = pick_product_asset(
            &self.options.product_name,
            &triple,
            &os_arch,
            &release.assets,
        )
        .cloned()
        .ok_or_else(|| UpdateError::MissingAsset {
            tag: release.tag_name.clone(),
            product: self.options.product_name.clone(),
            triple,
        })?;
        Ok(Some(UpdateAvailability {
            current: self.options.current_version.clone(),
            latest,
            tag: release.tag_name,
            release_url: release.html_url,
            asset,
        }))
    }

    /// Download the release asset and replace [`UpdateOptions::native_path`].
    pub fn apply(&self, availability: &UpdateAvailability) -> Result<()> {
        let client = self.client();
        let work = tempfile_dir()?;
        let staged = stage_release_asset(&client, &availability.asset, &work)?;
        replace_native_artifact(&staged, &self.options.native_path)?;
        Ok(())
    }
}

/// Parse a GitHub release tag into a [`Version`] (strips a leading `v`).
pub fn normalize_release_tag(tag: &str) -> Result<Version> {
    let trimmed = tag.trim().trim_start_matches(['v', 'V']);
    Ok(Version::parse(trimmed)?)
}

fn tempfile_dir() -> Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("wae-self-update-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| UpdateError::io(&dir, e))?;
    Ok(dir)
}
