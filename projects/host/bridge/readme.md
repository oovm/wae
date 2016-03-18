# host/bridge（Rust：`wae-bridge`）

类 Tauri 原生客户端壳：窗口、WebView、IPC、系统能力、生命周期。

- **不**编译进 TS 前端 bundle
- **不**负责 JSX / 路由 / UI / 前端状态
- 与 TS 仅通过 communication 协议 IPC 通信
