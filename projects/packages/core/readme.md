# `@wae/core`

client 与 server **共用的小型 runtime**：生成 ID、识别 `WaeError`、提取错误消息。不含 HTTP app、不含协议 JSON、不含 DOM/UI。

## 安装

```bash
pnpm add @wae/core@0.0.0
```

## API

| 函数 | 行为 |
|------|------|
| `createRequestId()` | 返回 `req_<time>_<rand>` |
| `createNodeId()` | 返回 `node_<rand>` |
| `isWaeError(value)` | 结构判别（含 `code` + `message`） |
| `toErrorMessage(error)` | `WaeError` / `Error` / 其它 → 字符串 |

## 消息语义（本包范围）

| 问题 | 答案（0.0.0） |
|------|----------------|
| request 是否有 ID | 由调用方或 `@wae/protocol.createRpcRequest` 生成；本包只提供生成器 |
| 取消 | **不**实现 Abort 编排；用标准 `AbortSignal` 自行传递 |
| 送达保证 | **无**；本包不负责传输 |
| 顺序 / 并发 | **无**队列语义 |

传输与至少一次送达属于 transport / host，不在 core。

## 相关

- 类型：[`@wae/types`](../public-types/readme.md)
- 编解码：[`@wae/protocol`](../protocol/readme.md)
