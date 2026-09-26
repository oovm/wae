# `wae-host`

Self-hosted WAE desktop orchestration: routes page IPC to window commands and `wae-bridge`.

```text
wae-desktop (bin)
  → wae-host::run_desktop
      → wae-platform-win32 (direct WebView2 + Win32)
      → wae-bridge::WebViewRuntime (protocol messages)
```

Non-window IPC strings are parsed as JSON `ClientMessage` when possible.
