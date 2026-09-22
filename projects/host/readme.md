# host

给 **Rust host / WebView 壳** 作者：本地应用壳如何与前端运行时通信。

这里不是 `@wae/server` 那种远程业务后端，也不是浏览器里的 `fetch`。host 解决的是：

```text
前端（TS） ↔ bridge ↔ Rust 壳 ↔ 窗口 / 文件系统 / 其它 native
```

## 目录

| 路径 | 说明 |
|------|------|
| [`bridge/`](bridge/readme.md) | crate `wae-bridge`：消息运输与 `WebViewRuntime` |

## 与其它层的关系

- 消息形状与 TS `@wae/protocol` / `@wae/types`、Rust `wae-types` 对齐为目标。
- 平台发行包在 [`../platform`](../platform/readme.md)；host 被平台壳加载，应用作者通常只装 `@wae/wae`。
- Rust **默认不**打进前端 bundle；Wasm 客户端走 `@wae/wae-unknown-wasm32` 显式路径。

## 怎么测（不启完整桌面壳）

```bash
cargo test -p wae-bridge
cargo check -p wae-bridge
```

详见 [`bridge/readme.md`](bridge/readme.md)。
