use std::path::{Path, PathBuf};

use crate::UpdatePackage;
use crate::apply::stage_release_asset;
use crate::error::Result;
use crate::github::GitHubClient;

/// Downloaded and extracted native addon awaiting apply.
#[derive(Debug, Clone)]
pub struct DownloadedUpdate {
    pub package: UpdatePackage,
    pub staged_native_path: PathBuf,
    pub work_dir: PathBuf,
}

pub fn download_package(client: &GitHubClient, package: &UpdatePackage, work_dir: &Path) -> Result<DownloadedUpdate> {
    let staged_native_path = stage_release_asset(client, &package.asset, work_dir)?;
    Ok(DownloadedUpdate { package: package.clone(), staged_native_path, work_dir: work_dir.to_path_buf() })
}
