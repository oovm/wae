use std::fs::File;
use std::io::{BufReader, copy};
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;

use crate::error::{Result, UpdateError};

pub fn extract_release_archive(archive: &Path, dest: &Path) -> Result<PathBuf> {
    let name = archive.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_ascii_lowercase();
    if name.ends_with(".zip") {
        extract_zip(archive, dest)?;
    } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        extract_tar_gz(archive, dest)?;
    } else {
        let target = dest.join(archive.file_name().unwrap_or_default());
        std::fs::copy(archive, &target).map_err(|e| UpdateError::io(&target, e))?;
        return Ok(target);
    }
    find_native_artifact(dest)
}

fn extract_zip(archive: &Path, dest: &Path) -> Result<()> {
    let file = File::open(archive).map_err(|e| UpdateError::io(archive, e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| UpdateError::Archive(format!("zip open: {e}")))?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| UpdateError::Archive(format!("zip entry: {e}")))?;
        let outpath = match entry.enclosed_name() {
            Some(path) => dest.join(path),
            None => continue,
        };
        if entry.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| UpdateError::io(&outpath, e))?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent).map_err(|e| UpdateError::io(parent, e))?;
            }
            let mut outfile = File::create(&outpath).map_err(|e| UpdateError::io(&outpath, e))?;
            copy(&mut entry, &mut outfile).map_err(|e| UpdateError::Archive(format!("zip copy: {e}")))?;
        }
    }
    Ok(())
}

fn extract_tar_gz(archive: &Path, dest: &Path) -> Result<()> {
    let file = File::open(archive).map_err(|e| UpdateError::io(archive, e))?;
    let gz = GzDecoder::new(BufReader::new(file));
    tar::Archive::new(gz).unpack(dest).map_err(|e| UpdateError::Archive(format!("tar unpack: {e}")))?;
    Ok(())
}

fn is_native_artifact(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_ascii_lowercase();
    name.ends_with(".node") || name.ends_with(".dll") || name.ends_with(".so") || name.ends_with(".dylib")
}

fn find_native_artifact(extract_root: &Path) -> Result<PathBuf> {
    let mut hits = Vec::new();
    collect_native_artifacts(extract_root, &mut hits)?;
    if hits.len() == 1 {
        return Ok(hits[0].clone());
    }
    if let Some(node) =
        hits.iter().find(|p| p.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("node")))
    {
        return Ok(node.clone());
    }
    Err(UpdateError::Archive("could not locate a single native addon (.node / .dll / .so / .dylib) in release archive".into()))
}

fn collect_native_artifacts(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).map_err(|e| UpdateError::io(dir, e))? {
        let entry = entry.map_err(|e| UpdateError::io(dir, e))?;
        let path = entry.path();
        if path.is_dir() {
            collect_native_artifacts(&path, out)?;
        } else if path.is_file() && is_native_artifact(&path) {
            out.push(path);
        }
    }
    Ok(())
}
