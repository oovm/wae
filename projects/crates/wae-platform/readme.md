# `wae-platform`

Platform-neutral host surface for WAE desktop shells.

| Type                 | Role                                              |
|----------------------|---------------------------------------------------|
| `DesktopOpenOptions` | URL, title, undecorated frame                     |
| `WindowCommand`      | minimize / maximize / move / close                |
| `DesktopIpcOutcome`  | window command, ignore, or reply JSON to the page |
| `DesktopIpcHandler`  | host callback for page → Rust IPC                 |

Low-level binding traits (`PlatformWindow`, `PlatformWebView`, `PlatformRuntime`) live in
[`src/runtime.rs`](src/runtime.rs). Concrete bindings live in `wae-platform-win32`, `wae-platform-darwin`,
`wae-platform-linux`.
