use semver::Version;
use wae_self_update::{GitHubReleaseAsset, normalize_release_tag, pick_product_asset};

#[test]
fn pick_asset_prefers_triple_zip() {
    let assets = vec![
        GitHubReleaseAsset {
            name: "my-app-x86_64-pc-windows-msvc.zip".into(),
            browser_download_url: "https://example.com/a.zip".into(),
            size: 1,
        },
        GitHubReleaseAsset {
            name: "other.zip".into(),
            browser_download_url: "https://example.com/b.zip".into(),
            size: 1,
        },
    ];
    let picked = pick_product_asset(
        "my-app",
        "x86_64-pc-windows-msvc",
        "windows-x86_64",
        &assets,
    );
    assert_eq!(picked.unwrap().name, "my-app-x86_64-pc-windows-msvc.zip");
}

#[test]
fn normalize_strips_v_prefix() {
    assert_eq!(
        normalize_release_tag("v1.2.3").unwrap(),
        Version::new(1, 2, 3)
    );
}
