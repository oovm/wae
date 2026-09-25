# host/desktop

桌面 **WebView 宿主**二进制 `wae-desktop`（crate `wae-desktop`）。

开发期由 `@wae/wae-win32-*` 等平台包拉起，加载 frontend 的 `devUrl`（通常是 Vite）。

```bash
cargo run -p wae-desktop -- --url http://127.0.0.1:5173/
```

Windows 需要已安装 [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)。
