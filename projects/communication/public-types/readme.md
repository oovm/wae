# `@wae/types`

跨包共享的 **TypeScript 类型**。无 runtime、无副作用；不要把本包当序列化库。

正式类型预期由 `wae generate types` 从 `projects/communication/schema`（crate `wae-types`）生成；**CLI 生成命令在 0.0.0 仍为骨架**，当前仓库内类型为手写对齐稿。

## 安装

```bash
pnpm add @wae/types@0.0.0
```

## 稳定共享面（当前）

| 类型 | 用途 |
|------|------|
| `NodeId` / `RequestId` / `RouteId` / `SessionId` | 标识符字符串别名 |
| `ErrorCode` / `WaeError` | 结构化错误 |
| `DomPatch` | 宿主 → 前端 DOM 补丁操作 |
| `UiEvent` | 前端 → 宿主 UI 事件 |
| `RpcRequest` / `RpcResponse` | RPC 信封 |
| `HostMessage` | 宿主下行消息联合类型 |

## 能否直接上线传输？

| 类型 | 网络 / 持久化 |
|------|----------------|
| `WaeError`、`RpcRequest`、`RpcResponse`、`HostMessage`、`DomPatch` | 设计为 JSON 可序列化；实际编解码用 `@wae/protocol` |
| `NodeId` 等别名 | 仅 `string`；语义约束在应用层 |

仅应用内使用的领域模型 **不要**塞进本包；本包只放跨 client / server / host 的契约。

## 版本

字段集合随 `0.0.0` 占位发布；破坏性变更会走协议与 codegen 流程。升级时优先看 `WaeError.code` 与 `HostMessage.type` 是否仍可辨识。
