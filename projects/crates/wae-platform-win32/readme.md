# `wae-platform-win32`

Direct **WebView2 + Win32** desktop binding for WAE. Does not use `tao` or `wry`.

| API      | Backing                                         |
|----------|-------------------------------------------------|
| Window   | Win32 (`CreateWindowExW`, `WndProc`)            |
| WebView  | Microsoft WebView2 (`ICoreWebView2*`)           |
| Page IPC | `WebMessageReceived` / `PostWebMessageAsString` |

Requires [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) on the machine.
