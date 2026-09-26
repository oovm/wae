use std::fs::{self, File};
use std::path::{Path, PathBuf};

use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::error::{BuildError, Result};
use crate::manifest::{load_product_manifest, resolve_native_path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageMode {
    /// Single native addon — matches `wae-updater` GitHub Release assets.
    NativeOnly,
    /// Full `wae build` product tree (frontend + lib + manifest + assets).
    FullProduct,
}

#[derive(Debug, Clone)]
pub struct PackageOptions {
    pub product_root: PathBuf,
    pub mode: PackageMode,
    pub archive_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PackageOutput {
    pub archive_path: PathBuf,
    pub entries: usize,
}

pub fn package_product(options: &PackageOptions) -> Result<PackageOutput> {
    if let Some(parent) = options.archive_path.parent() {
        fs::create_dir_all(parent).map_err(|e| BuildError::io(parent, e))?;
    }
    if options.archive_path.exists() {
        fs::remove_file(&options.archive_path).map_err(|e| BuildError::io(&options.archive_path, e))?;
    }

    let entries = match options.mode {
        PackageMode::NativeOnly => package_native_only(&options.product_root, &options.archive_path)?,
        PackageMode::FullProduct => package_full_tree(&options.product_root, &options.archive_path)?,
    };

    Ok(PackageOutput {
        archive_path: options.archive_path.clone(),
        entries,
    })
}

fn package_native_only(product_root: &Path, archive_path: &Path) -> Result<usize> {
    let manifest = load_product_manifest(product_root)?;
    let native = resolve_native_path(product_root, &manifest)?;
    if !native.is_file() {
        return Err(BuildError::MissingPath(native));
    }
    let file_name = native
        .file_name()
        .ok_or_else(|| BuildError::Package("native path has no file name".into()))?
        .to_owned();

    let file = File::create(archive_path).map_err(|e| BuildError::io(archive_path, e))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    zip.start_file(file_name.to_string_lossy(), options)
        .map_err(|e| BuildError::Package(e.to_string()))?;
    let mut src = File::open(&native).map_err(|e| BuildError::io(&native, e))?;
    std::io::copy(&mut src, &mut zip).map_err(|e| BuildError::Package(e.to_string()))?;
    zip.finish().map_err(|e| BuildError::Package(e.to_string()))?;
    Ok(1)
}

fn package_full_tree(product_root: &Path, archive_path: &Path) -> Result<usize> {
    let file = File::create(archive_path).map_err(|e| BuildError::io(archive_path, e))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    let mut count = 0usize;
    add_dir_to_zip(product_root, product_root, &mut zip, options, &mut count)?;
    zip.finish().map_err(|e| BuildError::Package(e.to_string()))?;
    Ok(count)
}

fn add_dir_to_zip(
    base: &Path,
    dir: &Path,
    zip: &mut ZipWriter<File>,
    options: SimpleFileOptions,
    count: &mut usize,
) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(|e| BuildError::io(dir, e))? {
        let entry = entry.map_err(|e| BuildError::io(dir, e))?;
        let path = entry.path();
        if path.is_dir() {
            add_dir_to_zip(base, &path, zip, options, count)?;
            continue;
        }
        let rel = path
            .strip_prefix(base)
            .map_err(|e| BuildError::Package(e.to_string()))?;
        let name = rel.to_string_lossy().replace('\\', "/");
        zip.start_file(name, options)
            .map_err(|e| BuildError::Package(e.to_string()))?;
        let mut src = File::open(&path).map_err(|e| BuildError::io(&path, e))?;
        std::io::copy(&mut src, zip).map_err(|e| BuildError::Package(e.to_string()))?;
        *count += 1;
    }
    Ok(())
}

pub fn default_native_archive_name(product_name: &str, triple: &str) -> String {
    format!("{product_name}-{triple}.zip")
}
