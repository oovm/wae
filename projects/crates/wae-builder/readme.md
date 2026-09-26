# wae-builder

Rust builder for **`wae build` products**: compile native addons, stage icons, verify the product tree, and package release archives.

| Step | API / CLI | Description |
|------|-----------|-------------|
| **Compile** | `compile_native` / `wae-builder compile` | `cargo build -p wae-napi` for a platform triple, optionally copy into `lib/` |
| **Icons** | `stage_icons` / `wae-builder icons` | Validate `.ico` / `.icns` / `.png` and write `assets/icons/` + `icon-manifest.json` |
| **Verify** | `Builder::verify_product_tree` / `wae-builder verify` | Check `wae-product.json`, `frontend/`, `lib/*.node` |
| **Package** | `package_product` / `wae-builder package` | Zip native-only (updater) or full product tree |

Pairs with [`wae-updater`](../wae-updater/readme.md): `PackageMode::NativeOnly` produces `{product}-{triple}.zip` assets consumed by product self-update.

## Icon config (`wae-builder.json`)

At the app project root:

```json
{
  "icons": {
    "windows": "assets/icon.ico",
    "macos": "assets/icon.icns",
    "linux": "assets/icon.png",
    "png": "assets/icon.png"
  }
}
```

Staged under `dist/<platform>/assets/icons/` in the built product.

## CLI

```bash
# Compile host native into platform package lib/ (monorepo dev)
cargo run -p wae-builder -- compile --workspace . --platform win32-x64

# After `wae build` — stage icons + package for GitHub Release
cargo run -p wae-builder -- icons --project . --product-root dist/win32-x64
cargo run -p wae-builder -- package --product-root dist/win32-x64 --out dist-release/demo-x86_64-pc-windows-msvc.zip

# Full pipeline (verify + optional compile + icons + native zip)
cargo run -p wae-builder -- all --workspace . --project . --product-root dist/win32-x64
```

`@wae/wae` `wae build` still owns frontend bundling and `wae-product.json`. Call `wae-builder` afterward for icons and distribution archives.
