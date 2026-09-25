# `@wae/protocol`

协议 **runtime**：把 client / host 消息编成 JSON 字符串，并构造 RPC 请求与响应信封。类型定义在 `@wae/types`；ID 生成依赖 `@wae/core`。

## 安装

```bash
pnpm add @wae/protocol@0.0.0
```

## 消息种类

**Client → Host（`ClientMessage`）**

| `type` | 含义 |
|--------|------|
| `uiEvent` | UI 事件 |
| `rpc` | RPC 请求（含 `RpcRequest`） |
| `native` | 原生 capability 调用 |

**Host → Client（`HostMessage`）**

| `type` | 含义 |
|--------|------|
| `domPatch` | DOM 补丁列表 |
| `rpc` | RPC 响应 |
| `error` | `WaeError` |

## API

```ts
import {
  encodeMessage,
  decodeClientMessage,
  decodeHostMessage,
  createRpcRequest,
  okRpcResponse,
  errRpcResponse,
  hostDomPatches,
} from "@wae/protocol";

const req = createRpcRequest("ping", { n: 1 });
const wire = encodeMessage({ type: "rpc", request: req });
const back = decodeClientMessage(wire);
```

| 函数 | 说明 |
|------|------|
| `encodeMessage` | `JSON.stringify` |
| `decode*` | `JSON.parse`（**无** schema 校验） |
| `createRpcRequest` | 自动 `createRequestId()` |
| `okRpcResponse` / `errRpcResponse` | 构造响应 |
| `hostDomPatches` | 包装 `DomPatch[]` |

## 版本 / 握手 / 兼容（现状）

| 项 | 0.0.0 |
|----|-------|
| 协议版本字段 | **无** 独立 version 帧 |
| 握手 | **无** |
| 兼容策略 | 依赖 JSON 形状；未知字段由 `JSON.parse` 保留但 TypeScript 类型不描述 |
| 运行时校验 | **无**；错误形状靠约定 |

扩展字段：在实现握手前，不要依赖未写入 `@wae/types` 的字段做生产兼容。

## 相关

- [`@wae/types`](../public-types/readme.md)
- Host：[`host/bridge`](../../crates/wae-bridge/readme.md)
