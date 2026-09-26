use std::fs;
use std::path::{Path, PathBuf};

use crate::archive::extract_release_archive;
use crate::error::{Result, UpdateError};
use crate::github::GitHubReleaseAsset;

pub fn stage_release_asset(
    client: &crate::github::GitHubClient,
    asset: &GitHubReleaseAsset,
    work_dir: &Path,
) -> Result<PathBuf> {
    fs::create_dir_all(work_dir).map_err(|e| UpdateError::io(work_dir, e))?;
    let archive_path = work_dir.join(&asset.name);
    client.download_asset(asset, &archive_path)?;
    let extract_dir = work_dir.join("extract");
    fs::create_dir_all(&extract_dir).map_err(|e| UpdateError::io(&extract_dir, e))?;
    extract_release_archive(&archive_path, &extract_dir)
}

/// Replace `native_path` with the staged native addon (`.node` / `.dll` / `.so` / `.dylib`).
///
/// On Windows a **loaded** native module usually cannot be overwritten; unload or restart first.
pub fn replace_native_artifact(staged: &Path, native_path: &Path) -> Result<()> {
    if let Some(parent) = native_path.parent() {
        fs::create_dir_all(parent).map_err(|e| UpdateError::io(parent, e))?;
    }

    let backup = backup_path(native_path);
    if native_path.exists() {
        if backup.exists() {
            fs::remove_file(&backup).map_err(|e| UpdateError::io(&backup, e))?;
        }
        fs::rename(native_path, &backup).map_err(|e| UpdateError::io(native_path, e))?;
    }

    match fs::rename(staged, native_path) {
        Ok(()) => Ok(()),
        Err(_) => {
            fs::copy(staged, native_path).map_err(|e| UpdateError::io(native_path, e))?;
            Ok(())
        }
    }
}

fn backup_path(path: &Path) -> PathBuf {
    let mut s = path.as_os_str().to_owned();
    s.push(".bak");
    PathBuf::from(s)
}
