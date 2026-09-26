# crates

Rust host and protocol schema; not included in the frontend bundle.

| crate                                                | Description                                                       |
|------------------------------------------------------|-------------------------------------------------------------------|
| [`wae-types`](wae-types/readme.md)                   | Cross-end messages and shared types (schema source)               |
| [`wae-bridge`](wae-bridge/readme.md)                 | WebView / native bridge runtime                                   |
| [`wae-platform`](wae-platform/readme.md)             | Platform-neutral host traits and desktop options                  |
| [`wae-platform-win32`](wae-platform-win32/readme.md) | Direct WebView2 + Win32 binding (no tao/wry)                      |
| [`wae-platform-darwin`](wae-platform-darwin/readme.md) | macOS WKWebView + AppKit (skeleton)                             |
| [`wae-platform-linux`](wae-platform-linux/readme.md) | Linux WebKitGTK + GTK (skeleton)                                  |
| [`wae-host`](wae-host/readme.md)                     | Self-hosted desktop orchestration                                 |
| [`wae-napi`](wae-napi/readme.md)                     | Node-API bindings for host (`openDesktop`, `handleClientMessage`) |
| [`wae-desktop`](wae-desktop/readme.md)               | Desktop dev binary (`--url`)                                      |
| [`wae-self-update`](wae-self-update/readme.md)       | GitHub Releases self-update for **wae build** products (`.node`)  |

Related npm packages: [`../packages`](../packages/readme.md). Examples: [`../examples`](../examples/readme.md).

```bash
cargo check --workspace
cargo test -p wae-host
```
