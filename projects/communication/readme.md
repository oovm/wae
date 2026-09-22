# communication

给**协议与跨端数据边界**作者：搞清一条消息从前端到 host / server 再回来要经过什么。

## 包与职责

| 路径 | npm / crate | 职责 |
|------|-------------|------|
| [`public-types/`](public-types/readme.md) | `@wae/types` | **仅类型**，无 runtime |
| [`core/`](core/readme.md) | `@wae/core` | ID 生成、错误判定等共享 runtime |
| [`protocol/`](protocol/readme.md) | `@wae/protocol` | JSON 编解码、RPC 构造 |
| [`schema/`](schema/readme.md) | Rust `wae-types` | schema / codegen 源（不发 npm） |

依赖方向：`types → core | protocol → client | server`。

## 消息生命周期（目标语义）

```text
createRpcRequest(method, args)     @wae/protocol
  → encodeMessage                  JSON 字符串
  → transport（HTTP / WS / IPC）
  → decode* / host handle
  → okRpcResponse / errRpcResponse / HostMessage
  → 对端解码并处理
```

| 种类 | 谁发 | 含义 |
|------|------|------|
| `uiEvent` | client | UI 事件上送 |
| `rpc` request | client | 带 `id` 的 RPC |
| `native` | client | 原生 capability 调用（需 host 鉴权） |
| `domPatch` | host | 补丁下发 |
| `rpc` response | host | 对应 `id` 的结果 |
| `error` | host | 协议/宿主错误 |

请求 **有 ID**（`createRequestId`）。取消、送达保证、握手与版本协商在 `0.0.0` **尚未实现为完整协议栈**——当前是可运行的 JSON 编解码与类型面。

读包 README 了解稳定字段与限制。
