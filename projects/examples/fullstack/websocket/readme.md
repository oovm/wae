# @wae-example/fullstack-websocket

## 这个示例展示什么

全栈 WebSocket 意图：client 与 server 同仓，目标是双向帧，而不仅是 HTTP request/response。源码尚未出现 WebSocket API。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/fullstack-websocket run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — 仍是 HTTP 向的双构造
- `package.json`

## 请求 / 事件路径（目标语义）

```text
browser WebSocket
  → 升级 / 独立 WS 端点（目标）
  → server 帧处理
  → 推送回 client
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 先把 HTTP `route` + `app.fetch` 跑通，再单独查运行时如何挂 WS（Node/Deno 各不相同）。
- 对照 `backend/typescript/websocket`：那边无 client，这边强调全栈配对。
- 勿把 `createServer()` 空调用误认为已监听 WS。

## 与生产应用的差异

生产要心跳、重连、鉴权握手；本示例无端口、无帧。

依赖（本示例）：`@wae/client`、`@wae/server`、`@wae/serverless`。
