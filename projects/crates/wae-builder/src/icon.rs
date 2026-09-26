use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::{BuildError, Result};
use crate::icon_pipeline::generate_icons_from_source;

pub const ICON_MANIFEST_FILE: &str = "icon-manifest.json";
pub const ICONS_DIR: &str = "assets/icons";
pub const SIZES_SUBDIR: &str = "sizes";

/// User-provided icon source. Only PNG or SVG — platform formats are generated.
#[derive(Debug, Clone)]
pub struct IconSources {
    pub source: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IconManifest {
    pub source: String,
    pub windows: String,
    pub macos: String,
    pub linux: String,
    pub png: String,
    pub sizes: Vec<u32>,
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
    let root: serde_json::Value = serde_json::from_str(&text).map_err(|e| BuildError::Icon(e.to_string()))?;
    let icons = root.get("icons").cloned().unwrap_or(root);
    let source = parse_icon_source_value(&icons)?;
    let abs = resolve_source_path(project_root, &source)?;
    Ok(Some(IconSources { source: abs }))
}

fn parse_icon_source_value(value: &serde_json::Value) -> Result<PathBuf> {
    match value {
        serde_json::Value::String(path) => Ok(PathBuf::from(path)),
        serde_json::Value::Object(map) => {
            for key in ["source", "svg", "png"] {
                if let Some(v) = map.get(key).and_then(|v| v.as_str()) {
                    return Ok(PathBuf::from(v));
                }
            }
            Err(BuildError::Icon("icons object needs `source`, `svg`, or `png` path".into()))
        }
        _ => Err(BuildError::Icon("icons must be a path string or `{ \"source\": \"...\" }` object".into())),
    }
}

fn resolve_source_path(project_root: &Path, source: &Path) -> Result<PathBuf> {
    let abs = if source.is_absolute() { source.to_path_buf() } else { project_root.join(source) };
    if !abs.is_file() {
        return Err(BuildError::Icon(format!("icon not found: {}", abs.display())));
    }
    let ext = abs.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase());
    match ext.as_deref() {
        Some("png") | Some("svg") => Ok(abs),
        _ => Err(BuildError::Icon(format!("icon source must be .png or .svg (got {})", abs.display()))),
    }
}

pub fn stage_icons(project_root: &Path, product_root: &Path, sources: &IconSources) -> Result<IconStageResult> {
    let rel_source = project_relative(project_root, &sources.source);
    let out_dir = product_root.join(ICONS_DIR);
    fs::create_dir_all(&out_dir).map_err(|e| BuildError::io(&out_dir, e))?;

    let generated = generate_icons_from_source(&sources.source)?;
    let mut staged = Vec::new();

    let ico_path = out_dir.join("app.ico");
    write_bytes(&ico_path, &generated.ico)?;
    staged.push(ico_path);

    let icns_path = out_dir.join("app.icns");
    write_bytes(&icns_path, &generated.icns)?;
    staged.push(icns_path);

    let linux_path = out_dir.join("app-linux.png");
    write_bytes(&linux_path, &generated.linux_png)?;
    staged.push(linux_path);

    let png_path = out_dir.join("app.png");
    write_bytes(&png_path, &generated.app_png)?;
    staged.push(png_path);

    let sizes_dir = out_dir.join(SIZES_SUBDIR);
    fs::create_dir_all(&sizes_dir).map_err(|e| BuildError::io(&sizes_dir, e))?;
    let mut sizes = Vec::new();
    for (size, bytes) in &generated.size_pngs {
        let path = sizes_dir.join(format!("{size}.png"));
        write_bytes(&path, bytes)?;
        staged.push(path);
        sizes.push(*size);
    }

    let manifest = IconManifest {
        source: rel_source,
        windows: format!("{ICONS_DIR}/app.ico"),
        macos: format!("{ICONS_DIR}/app.icns"),
        linux: format!("{ICONS_DIR}/app-linux.png"),
        png: format!("{ICONS_DIR}/app.png"),
        sizes,
    };
    let manifest_path = out_dir.join(ICON_MANIFEST_FILE);
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| BuildError::Icon(e.to_string()))?;
    fs::write(&manifest_path, format!("{json}\n")).map_err(|e| BuildError::io(&manifest_path, e))?;
    staged.push(manifest_path.clone());

    Ok(IconStageResult { manifest_path, staged })
}

fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).map_err(|e| BuildError::io(path, e))
}

fn project_relative(project_root: &Path, abs: &Path) -> String {
    abs.strip_prefix(project_root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| abs.to_string_lossy().replace('\\', "/"))
}
