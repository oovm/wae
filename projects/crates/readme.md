# crates

Rust 宿主与协议 schema，不进前端 bundle。

| crate | 说明 |
|-------|------|
| [`wae-types`](wae-types/readme.md) | 跨端消息与共享类型（schema 源） |
| [`wae-bridge`](wae-bridge/readme.md) | WebView / native 桥接运行时 |
| [`wae-desktop`](wae-desktop/readme.md) | 桌面 WebView 壳（开发期加载 frontend URL） |

相关 npm 包见 [`../packages`](../packages/readme.md)，示例见 [`../examples`](../examples/readme.md)。

```bash
cargo check --workspace
cargo test -p wae-bridge
```
