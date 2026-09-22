# `wae-bridge`（host/bridge）

Rust crate：WebView / WASM / native 宿主与前端之间的 **桥**。配合 `@wae/client`、`@wae/protocol` 与 `@wae/wae-*`。

**不是** npm 包；`publish = false`。

## 调用方向

```text
ClientMessage（前端）
  → MessageTransport::send_json / 宿主收包
  → WebViewRuntime::handle
  → Option<HostMessage>（回前端）
```

| API | 含义 |
|-----|------|
| `MessageTransport` | `send_json(&str) -> Result<()>` |
| `WebViewRuntime::handle` | 处理 `ClientMessage`；**0.0.0 恒返回 `None`** |
| `WebViewBridge` | 生命周期占位 |

同步 / 异步：当前 `handle` 为同步签名；真实 WebView 回调多为异步，接线时再定。

## 序列化与错误

- 消息类型来自 crate `wae-types`（schema）
- JSON 载荷与 TS `@wae/protocol` 对齐为目标；改形状需两侧一起改
- 权限：native capability 应在 host 侧鉴权；前端 `native` 消息不可默认可信

## 不启动完整桌面壳时如何测

```bash
cargo test -p wae-bridge
cargo check -p wae-bridge
```

对 `WebViewRuntime::handle` 注入样例 `ClientMessage`，断言返回的 `HostMessage`（待实现后）。

## 相关

- [`@wae/protocol`](../../communication/protocol/readme.md)
- 平台包：[`platform`](../../platform/readme.md)
