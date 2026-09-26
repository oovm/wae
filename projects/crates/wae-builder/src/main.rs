//! `wae-builder` CLI — compile, icons, package for WAE products.

use std::env;
use std::path::PathBuf;
use std::process;

use wae_builder::{
    Builder, BuilderConfig, CompileOptions, PackageMode, PackagePlan, PackageOptions, compile_native,
    default_native_archive_name, host_platform, load_icon_sources, load_product_manifest,
    package_product, platform_by_id, resolve_native_path, stage_icons,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("wae-builder: {err}");
        process::exit(1);
    }
}

fn run() -> wae_builder::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "help" || args[0] == "--help" {
        print_help();
        return Ok(());
    }
    match args[0].as_str() {
        "compile" => cmd_compile(&args[1..]),
        "icons" => cmd_icons(&args[1..]),
        "package" => cmd_package(&args[1..]),
        "verify" => cmd_verify(&args[1..]),
        "all" => cmd_all(&args[1..]),
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            process::exit(1);
        }
    }
}

fn print_help() {
    eprintln!(
        "wae-builder — compile, icons, package WAE products\n\n\
Commands:\n\
  compile   --workspace <dir> [--platform <id>] [--product-root <dir>] [--debug]\n\
  icons     --project <dir> --product-root <dir> [--config wae-builder.json]\n\
  package   --product-root <dir> [--mode native|full] [--out <zip>]\n\
  verify    --product-root <dir>\n\
  all       --workspace <dir> --project <dir> --product-root <dir> [--package-out <zip>]\n"
    );
}

fn cmd_compile(args: &[String]) -> wae_builder::Result<()> {
    let workspace = arg_path(args, "--workspace")?;
    let platform_id = match arg_string(args, "--platform") {
        Some(id) => id,
        None => host_platform()?.id.to_string(),
    };
    let platform = platform_by_id(&platform_id)?;
    let release = !args.iter().any(|a| a == "--debug");
    let dest = if let Some(root) = optional_path(args, "--product-root") {
        let manifest = load_product_manifest(&root)?;
        Some(resolve_native_path(&root, &manifest)?)
    } else {
        None
    };

    let output = compile_native(&CompileOptions {
        workspace_root: workspace,
        platform: *platform,
        release,
        dest_lib: dest,
    })?;
    println!(
        "compiled {} {}",
        output.artifact.display(),
        output
            .installed_lib
            .as_ref()
            .map(|p| format!("→ {}", p.display()))
            .unwrap_or_default()
    );
    Ok(())
}

fn cmd_icons(args: &[String]) -> wae_builder::Result<()> {
    let project = arg_path(args, "--project")?;
    let product_root = arg_path(args, "--product-root")?;
    let sources = load_icon_sources(&project)?.ok_or_else(|| {
        wae_builder::BuildError::Icon("missing wae-builder.json icons section".into())
    })?;
    let result = stage_icons(&project, &product_root, &sources)?;
    println!("staged {} icon file(s) → {}", result.staged.len(), result.manifest_path.display());
    Ok(())
}

fn cmd_package(args: &[String]) -> wae_builder::Result<()> {
    let product_root = arg_path(args, "--product-root")?;
    let mode = match arg_string(args, "--mode").as_deref() {
        Some("full") => PackageMode::FullProduct,
        _ => PackageMode::NativeOnly,
    };
    let manifest = load_product_manifest(&product_root)?;
    let platform = platform_by_id(&manifest.platform)?;
    let archive = optional_path(args, "--out").unwrap_or_else(|| {
        PathBuf::from(default_native_archive_name(&manifest.name, platform.triple))
    });
    let output = package_product(&PackageOptions {
        product_root,
        mode,
        archive_path: archive,
    })?;
    println!(
        "packaged {} entries → {}",
        output.entries,
        output.archive_path.display()
    );
    Ok(())
}

fn cmd_verify(args: &[String]) -> wae_builder::Result<()> {
    let product_root = arg_path(args, "--product-root")?;
    let platform_id = load_product_manifest(&product_root)?.platform;
    let builder = Builder::new(BuilderConfig {
        workspace_root: PathBuf::from("."),
        product_root,
        project_root: PathBuf::from("."),
        platform_id,
        compile_native: false,
        release: true,
        stage_icons: false,
        icon_sources: None,
        package: None,
    });
    let manifest = builder.verify_product_tree()?;
    println!(
        "ok {}@{} platform={}",
        manifest.name, manifest.version, manifest.platform
    );
    Ok(())
}

fn cmd_all(args: &[String]) -> wae_builder::Result<()> {
    let workspace = arg_path(args, "--workspace")?;
    let project = arg_path(args, "--project")?;
    let product_root = arg_path(args, "--product-root")?;
    let manifest = load_product_manifest(&product_root)?;
    let platform = platform_by_id(&manifest.platform)?;
    let package_out = arg_string(args, "--package-out").map(|name| PackagePlan {
        mode: PackageMode::NativeOnly,
        archive_path: PathBuf::from(name),
    });

    let icon_sources = load_icon_sources(&project).ok().flatten();
    let stage_icons = icon_sources.is_some();

    let report = Builder::new(BuilderConfig {
        workspace_root: workspace,
        product_root: product_root.clone(),
        project_root: project,
        platform_id: platform.id.to_string(),
        compile_native: args.iter().any(|a| a == "--compile"),
        release: !args.iter().any(|a| a == "--debug"),
        stage_icons,
        icon_sources,
        package: package_out.or_else(|| {
            Some(PackagePlan {
                mode: PackageMode::NativeOnly,
                archive_path: PathBuf::from(default_native_archive_name(&manifest.name, platform.triple)),
            })
        }),
    })
    .run()?;

    if let Some(compiled) = &report.compiled {
        println!("compiled {}", compiled.artifact.display());
    }
    if let Some(icons) = &report.icons {
        println!("icons → {}", icons.manifest_path.display());
    }
    if let Some(package) = &report.package {
        println!("package → {}", package.archive_path.display());
    }
    Ok(())
}

fn arg_string(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_path(args: &[String], flag: &str) -> wae_builder::Result<PathBuf> {
    optional_path(args, flag)
        .ok_or_else(|| wae_builder::BuildError::Other(format!("missing {flag}")))
}

fn optional_path(args: &[String], flag: &str) -> Option<PathBuf> {
    arg_string(args, flag).map(PathBuf::from)
}
