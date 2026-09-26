# wae-self-update

Rust library for **`wae build` products** to check and apply updates from **GitHub Releases**.

| Layer | Upgrade path |
|-------|----------------|
| **WAE toolchain** (`@wae/wae`, dev deps) | npm / pnpm — **not** this crate |
| **Shipped app** (your product) | GitHub Releases → native addon + bundle |

The native host is the **`wae-napi` dynamic library** (`.node` on all platforms via napi-rs), embedded in the built app tree — not a `wae` CLI binary.

## Usage (in your product)

```rust
use semver::Version;
use wae_self_update::{GitHubReleaseSource, SelfUpdate, UpdateOptions};

let update = SelfUpdate::new(UpdateOptions {
    source: GitHubReleaseSource::new("your-org", "your-app"),
    current_version: Version::parse("1.0.0")?,
    product_name: "your-app".into(),
    native_path: app_dir.join("lib/win32-x64-msvc.node"),
    target_triple: None,
    token: None,
    allow_prerelease: false,
    tag: None,
});

if let Some(availability) = update.check()? {
    update.apply(&availability)?;
}
```

From TypeScript (`@wae/wae` product helpers): `checkProductUpdateFromManifest` / `applyProductUpdateFromManifest`.

## GitHub Release assets

Publish **your app's** releases (not `oovm/wae`). One archive per target triple:

| Pattern | Example |
|---------|---------|
| `{product}-{triple}.zip` | `my-app-x86_64-pc-windows-msvc.zip` |
| `{product}-native-{triple}.zip` | `my-app-native-aarch64-apple-darwin.zip` |

Archive contents: the platform-specific **`wae-napi` `.node`** (e.g. `win32-x64-msvc.node`, `darwin-arm64.node`, `linux-x64-gnu.node`).

CI helper (monorepo dev only):

```bash
pnpm pack:product-native -- --product my-app
```

## Windows note

Replacing a **loaded** native module may fail. Restart the app or run apply before loading the addon.
