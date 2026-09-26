//! N-API: GitHub Releases self-update for **`wae build` products** (native addon).

use std::path::PathBuf;

use napi::bindgen_prelude::*;
use napi_derive::napi;
use semver::Version;
use wae_self_update::{GitHubReleaseSource, SelfUpdate, UpdateOptions};

#[napi(object)]
pub struct ProductUpdateOptions {
    /// GitHub `owner/repo` for **your shipped app**, not the WAE framework repo.
    pub repo: String,
    /// Product slug in release asset names (`{product_name}-{triple}.zip`).
    pub product_name: String,
    /// Installed native addon path (e.g. `lib/win32-x64-msvc.node` in the built app).
    pub native_path: String,
    pub current_version: String,
    pub tag: Option<String>,
    pub allow_prerelease: Option<bool>,
}

#[napi(object)]
pub struct ProductUpdateStatus {
    pub up_to_date: bool,
    pub current: String,
    pub latest: Option<String>,
    pub tag: Option<String>,
    pub release_url: Option<String>,
    pub asset_name: Option<String>,
}

fn build_update(options: &ProductUpdateOptions) -> std::result::Result<SelfUpdate, String> {
    let source = GitHubReleaseSource::from_slug(&options.repo).map_err(|e| e.to_string())?;
    let current_version = Version::parse(&options.current_version).map_err(|e| e.to_string())?;

    Ok(SelfUpdate::new(UpdateOptions {
        source,
        current_version,
        product_name: options.product_name.clone(),
        native_path: PathBuf::from(&options.native_path),
        target_triple: None,
        token: None,
        allow_prerelease: options.allow_prerelease.unwrap_or(false),
        tag: options.tag.clone(),
    }))
}

#[napi]
pub fn check_product_update(options: ProductUpdateOptions) -> Result<ProductUpdateStatus> {
    let update = build_update(&options).map_err(|e| Error::from_reason(e))?;
    match update.check().map_err(|e| Error::from_reason(e.to_string()))? {
        None => Ok(ProductUpdateStatus {
            up_to_date: true,
            current: options.current_version,
            latest: None,
            tag: None,
            release_url: None,
            asset_name: None,
        }),
        Some(availability) => Ok(ProductUpdateStatus {
            up_to_date: false,
            current: availability.current.to_string(),
            latest: Some(availability.latest.to_string()),
            tag: Some(availability.tag),
            release_url: Some(availability.release_url),
            asset_name: Some(availability.asset.name),
        }),
    }
}

#[napi]
pub fn apply_product_update(options: ProductUpdateOptions) -> Result<ProductUpdateStatus> {
    if options.native_path.is_empty() {
        return Err(Error::from_reason("native_path is required to apply a product update"));
    }
    let update = build_update(&options).map_err(|e| Error::from_reason(e))?;
    let availability = update
        .check()
        .map_err(|e| Error::from_reason(e.to_string()))?
        .ok_or_else(|| Error::from_reason("already up to date"))?;
    update
        .apply(&availability)
        .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(ProductUpdateStatus {
        up_to_date: true,
        current: availability.latest.to_string(),
        latest: Some(availability.latest.to_string()),
        tag: Some(availability.tag),
        release_url: Some(availability.release_url),
        asset_name: Some(availability.asset.name),
    })
}
