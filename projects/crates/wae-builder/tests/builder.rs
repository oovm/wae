use std::fs;

use image::{ImageBuffer, Rgba};
use tempfile::tempdir;
use wae_builder::{
    IconSources, PackageMode, PackageOptions, ProductManifest, WAE_PRODUCT_MANIFEST, package_product, platform_by_id,
    stage_icons,
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

fn write_test_png(path: &std::path::Path, width: u32, height: u32) {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_fn(width, height, |x, y| if x == y { Rgba([255, 0, 0, 255]) } else { Rgba([0, 0, 0, 0]) });
    img.save(path).unwrap();
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
fn stage_icons_generates_platform_formats_from_png() {
    let project = tempdir().unwrap();
    let product = tempdir().unwrap();
    write_product_tree(product.path(), "darwin-arm64");

    let png = project.path().join("icon.png");
    write_test_png(&png, 128, 128);

    let result = stage_icons(project.path(), product.path(), &IconSources { source: png }).unwrap();

    let icons = product.path().join("assets/icons");
    assert!(icons.join("app.ico").is_file());
    assert!(icons.join("app.icns").is_file());
    assert!(icons.join("app-linux.png").is_file());
    assert!(icons.join("app.png").is_file());
    assert!(icons.join("sizes/256.png").is_file());
    assert!(result.manifest_path.is_file());
    let text = fs::read_to_string(&result.manifest_path).unwrap();
    assert!(text.contains("\"sizes\""));
}

#[test]
fn stage_icons_generates_from_svg() {
    let project = tempdir().unwrap();
    let product = tempdir().unwrap();
    write_product_tree(product.path(), "win32-x64");

    let svg = project.path().join("icon.svg");
    fs::write(
        &svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 64 64\"><rect width=\"64\" height=\"64\" fill=\"#3366ff\"/></svg>",
    )
    .unwrap();

    stage_icons(project.path(), product.path(), &IconSources { source: svg }).unwrap();

    assert!(product.path().join("assets/icons/app.ico").is_file());
}
