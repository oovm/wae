use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{BuildError, Result};

pub const WAE_PRODUCT_MANIFEST: &str = "wae-product.json";

#[derive(Debug, Clone, serde::Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductManifest {
    pub schema_version: u32,
    pub name: String,
    pub version: String,
    pub platform: String,
    pub native_path: Option<String>,
    pub frontend_dir: Option<String>,
}

pub fn load_product_manifest(product_root: &Path) -> Result<ProductManifest> {
    let path = product_root.join(WAE_PRODUCT_MANIFEST);
    if !path.is_file() {
        return Err(BuildError::MissingManifest(path));
    }
    let text = fs::read_to_string(&path).map_err(|e| BuildError::io(&path, e))?;
    let manifest: ProductManifest =
        serde_json::from_str(&text).map_err(|e| BuildError::InvalidManifest(e.to_string()))?;
    if manifest.schema_version != 1 {
        return Err(BuildError::InvalidManifest(format!(
            "unsupported schemaVersion {}",
            manifest.schema_version
        )));
    }
    Ok(manifest)
}

pub fn resolve_native_path(product_root: &Path, manifest: &ProductManifest) -> Result<PathBuf> {
    let rel = manifest
        .native_path
        .as_deref()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            BuildError::InvalidManifest("nativePath missing for native product".into())
        })?;
    Ok(product_root.join(rel))
}

pub fn resolve_frontend_dir(product_root: &Path, manifest: &ProductManifest) -> PathBuf {
    product_root.join(manifest.frontend_dir.as_deref().unwrap_or("frontend"))
}
