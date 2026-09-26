use std::fs;
use tempfile::tempdir;
use wae_builder::{
    IconSources, PackageMode, PackageOptions, ProductManifest, WAE_PRODUCT_MANIFEST, package_product,
    platform_by_id, stage_icons,
};

fn write_product_tree(root: &std::path::Path, platform: &str) {
    let spec = platform_by_id(platform).unwrap();
    fs::create_dir_all(root.join("frontend")).unwrap();
    fs::write(root.join("frontend/index.html"), "<html></html>").unwrap();
    fs::create_dir_all(root.join("lib")).unwrap();
    let native = root.join("lib").join(spec.lib_file);
    fs::write(&native, b"fake-native").unwrap();
    let manifest = ProductManifest {
        schema_version: 1,
        name: "demo-app".into(),
        version: "1.0.0".into(),
        platform: platform.into(),
        native_path: Some(format!("lib/{}", spec.lib_file)),
        frontend_dir: Some("frontend".into()),
    };
    let json = serde_json::to_string_pretty(&manifest).unwrap();
    fs::write(root.join(WAE_PRODUCT_MANIFEST), format!("{json}\n")).unwrap();
}

#[test]
fn package_native_only_zip() {
    let dir = tempdir().unwrap();
    write_product_tree(dir.path(), "win32-x64");
    let archive = dir.path().join("demo-app-x86_64-pc-windows-msvc.zip");
    let output = package_product(&PackageOptions {
        product_root: dir.path().to_path_buf(),
        mode: PackageMode::NativeOnly,
        archive_path: archive.clone(),
    })
    .unwrap();
    assert!(output.archive_path.is_file());
    assert_eq!(output.entries, 1);
}

#[test]
fn stage_icons_writes_manifest() {
    let project = tempdir().unwrap();
    let product = tempdir().unwrap();
    write_product_tree(product.path(), "darwin-arm64");

    let png = project.path().join("icon.png");
    let bytes = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0];
    fs::write(&png, bytes).unwrap();

    let result = stage_icons(
        project.path(),
        product.path(),
        &IconSources {
            png: Some(png),
            windows: None,
            macos: None,
            linux: None,
        },
    )
    .unwrap();
    assert!(result.manifest_path.is_file());
    let text = fs::read_to_string(&result.manifest_path).unwrap();
    assert!(text.contains("app.png"));
}
