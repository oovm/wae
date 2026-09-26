use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::{BuildError, Result};

pub const ICON_MANIFEST_FILE: &str = "icon-manifest.json";
pub const ICONS_DIR: &str = "assets/icons";

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconSources {
    pub windows: Option<PathBuf>,
    pub macos: Option<PathBuf>,
    pub linux: Option<PathBuf>,
    pub png: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IconManifest {
    pub windows: Option<String>,
    pub macos: Option<String>,
    pub linux: Option<String>,
    pub png: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IconStageResult {
    pub manifest_path: PathBuf,
    pub staged: Vec<PathBuf>,
}

pub fn load_icon_sources(project_root: &Path) -> Result<Option<IconSources>> {
    let path = project_root.join("wae-builder.json");
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(|e| BuildError::io(&path, e))?;
    let root: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| BuildError::Icon(e.to_string()))?;
    let icons = root.get("icons").cloned().unwrap_or(root);
    let sources: IconSources =
        serde_json::from_value(icons).map_err(|e| BuildError::Icon(e.to_string()))?;
    Ok(Some(sources))
}

pub fn stage_icons(
    project_root: &Path,
    product_root: &Path,
    sources: &IconSources,
) -> Result<IconStageResult> {
    let out_dir = product_root.join(ICONS_DIR);
    fs::create_dir_all(&out_dir).map_err(|e| BuildError::io(&out_dir, e))?;

    let mut manifest = IconManifest {
        windows: None,
        macos: None,
        linux: None,
        png: None,
    };
    let mut staged = Vec::new();

    if let Some(src) = &sources.windows {
        let rel = stage_one(project_root, &out_dir, src, "app.ico", validate_ico)?;
        manifest.windows = Some(rel);
        staged.push(out_dir.join("app.ico"));
    }
    if let Some(src) = &sources.macos {
        let rel = stage_one(project_root, &out_dir, src, "app.icns", validate_icns)?;
        manifest.macos = Some(rel);
        staged.push(out_dir.join("app.icns"));
    }
    if let Some(src) = &sources.linux {
        let rel = stage_one(project_root, &out_dir, src, "app-linux.png", validate_png)?;
        manifest.linux = Some(rel);
        staged.push(out_dir.join("app-linux.png"));
    }
    if let Some(src) = &sources.png {
        let rel = stage_one(project_root, &out_dir, src, "app.png", validate_png)?;
        manifest.png = Some(rel);
        staged.push(out_dir.join("app.png"));
    }

    if staged.is_empty() {
        return Err(BuildError::Icon("no icon sources configured".into()));
    }

    let manifest_path = out_dir.join(ICON_MANIFEST_FILE);
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| BuildError::Icon(e.to_string()))?;
    fs::write(&manifest_path, format!("{json}\n")).map_err(|e| BuildError::io(&manifest_path, e))?;
    staged.push(manifest_path.clone());

    Ok(IconStageResult {
        manifest_path,
        staged,
    })
}

fn stage_one(
    project_root: &Path,
    out_dir: &Path,
    src: &Path,
    dest_name: &str,
    validate: fn(&Path) -> Result<()>,
) -> Result<String> {
    let abs = if src.is_absolute() {
        src.to_path_buf()
    } else {
        project_root.join(src)
    };
    if !abs.is_file() {
        return Err(BuildError::Icon(format!("icon not found: {}", abs.display())));
    }
    validate(&abs)?;
    let dest = out_dir.join(dest_name);
    fs::copy(&abs, &dest).map_err(|e| BuildError::io(&dest, e))?;
    Ok(format!("{ICONS_DIR}/{dest_name}"))
}

fn validate_ico(path: &Path) -> Result<()> {
    let bytes = fs::read(path).map_err(|e| BuildError::io(path, e))?;
    if bytes.len() < 6 || bytes[0] != 0 || bytes[1] != 0 {
        return Err(BuildError::Icon(format!("{} is not a valid ICO file", path.display())));
    }
    Ok(())
}

fn validate_icns(path: &Path) -> Result<()> {
    let bytes = fs::read(path).map_err(|e| BuildError::io(path, e))?;
    if bytes.len() < 8 || &bytes[0..4] != b"icns" {
        return Err(BuildError::Icon(format!("{} is not a valid ICNS file", path.display())));
    }
    Ok(())
}

fn validate_png(path: &Path) -> Result<()> {
    let bytes = fs::read(path).map_err(|e| BuildError::io(path, e))?;
    if bytes.len() < 8 || bytes[0..8] != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
        return Err(BuildError::Icon(format!("{} is not a valid PNG file", path.display())));
    }
    Ok(())
}
