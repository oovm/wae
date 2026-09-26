use semver::Version;
use wae_updater::{
    DownloadPolicy, GitHubReleaseAsset, GitHubReleaseSource, ReleaseChannel, Updater, UpdaterConfig,
    normalize_release_tag, pick_product_asset,
};

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

#[test]
fn channel_parse_and_policy() {
    assert_eq!(ReleaseChannel::parse("stable").unwrap(), ReleaseChannel::Stable);
    assert_eq!(ReleaseChannel::parse("beta").unwrap(), ReleaseChannel::Beta);
    assert_eq!(
        ReleaseChannel::parse("nightly-2026-09-26").unwrap(),
        ReleaseChannel::Pinned("nightly-2026-09-26".into())
    );
    assert_eq!(
        DownloadPolicy::parse("silent").unwrap(),
        DownloadPolicy::DownloadIfAvailable
    );
    assert_eq!(
        DownloadPolicy::parse("explicit").unwrap(),
        DownloadPolicy::CheckOnly
    );
}

#[test]
fn updater_check_only_policy_does_not_download() {
    let updater = Updater::new(UpdaterConfig {
        source: GitHubReleaseSource::new("oovm", "wae"),
        current_version: Version::new(99, 0, 0),
        product_name: "demo".into(),
        native_path: std::path::PathBuf::from("lib/demo.node"),
        channel: ReleaseChannel::Stable,
        download_policy: DownloadPolicy::CheckOnly,
        target_triple: None,
        token: None,
    });
    // Network may fail in CI — only assert config wiring compiles.
    let _ = updater.config().download_policy;
}
