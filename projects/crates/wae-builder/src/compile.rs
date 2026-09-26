use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{BuildError, Result};
use crate::platform::PlatformSpec;

#[derive(Debug, Clone)]
pub struct CompileOptions {
    pub workspace_root: PathBuf,
    pub platform: PlatformSpec,
    pub release: bool,
    pub dest_lib: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct CompileOutput {
    pub artifact: PathBuf,
    pub installed_lib: Option<PathBuf>,
}

pub fn compile_native(options: &CompileOptions) -> Result<CompileOutput> {
    let host = wae_updater::host_triple();
    let target = options.platform.triple;
    if target != host {
        rustup_target_add(target)?;
    }

    let mut cmd = Command::new("cargo");
    cmd.current_dir(&options.workspace_root);
    cmd.arg("build").arg("-p").arg("wae-napi");
    if options.release {
        cmd.arg("--release");
    }
    if target != host {
        cmd.arg("--target").arg(target);
    }
    let status = cmd.status().map_err(|e| BuildError::Cargo(e.to_string()))?;
    if !status.success() {
        return Err(BuildError::Cargo(format!("cargo build -p wae-napi exited with {}", status.code().unwrap_or(-1))));
    }

    let artifact = resolve_cargo_artifact(&options.workspace_root, target, options.release)?;
    let installed_lib = if let Some(dest) = &options.dest_lib {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(|e| BuildError::io(parent, e))?;
        }
        fs::copy(&artifact, dest).map_err(|e| BuildError::io(dest, e))?;
        Some(dest.clone())
    } else {
        None
    };

    Ok(CompileOutput { artifact, installed_lib })
}

fn rustup_target_add(target: &str) -> Result<()> {
    let status =
        Command::new("rustup").args(["target", "add", target]).status().map_err(|e| BuildError::Cargo(e.to_string()))?;
    if !status.success() {
        return Err(BuildError::Cargo(format!("rustup target add {target} failed")));
    }
    Ok(())
}

fn dylib_basename(triple: &str) -> &'static str {
    if triple.contains("windows") {
        "wae_napi.dll"
    } else if triple.contains("darwin") || triple.contains("ios") {
        "wae_napi.dylib"
    } else {
        "wae_napi.so"
    }
}

fn resolve_cargo_artifact(workspace_root: &Path, triple: &str, release: bool) -> Result<PathBuf> {
    let host = wae_updater::host_triple();
    let profile = if release { "release" } else { "debug" };
    let base = workspace_root.join("target");
    let dir = if triple == host { base.join(profile) } else { base.join(triple).join(profile) };
    let file = dir.join(dylib_basename(triple));
    if file.is_file() {
        return Ok(file);
    }
    Err(BuildError::Cargo(format!("artifact not found at {}", file.display())))
}
