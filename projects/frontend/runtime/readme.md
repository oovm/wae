# `@wae/client`

框架无关前端运行时。纯 TS / DOM 直接使用；Vue / React / Svelte / Solid 经独立 `@wae/adapter-*` 注入，**不要**在本包找组件或 hooks。

## 安装

```bash
pnpm add @wae/client@0.0.0
```

## `createClient` 创建什么

```ts
import { createClient } from "@wae/client";

const client = createClient({
  server: { baseUrl: "/api" }, // 必填
});

// client.env          运行环境探测结果
// client.server       HTTP / action 客户端
// client.native.bridge 浏览器或 native IPC bridge
// client.session      默认内存 session
// client.lifecycle    应用生命周期钩子
// client.navigation   默认浏览器导航
// client.connectWs(url) → WebSocket 客户端
```

可选覆盖：`env`、`session`、`lifecycle`、`navigation`、`bridge`。

## 怎样发请求

```ts
const res = await client.server.fetch("/hello");
const data = await res.json();

const greet = client.server.action<{ name: string }, { ok: boolean }>("greet");
const out = await greet.execute({ name: "wae" });
// POST {baseUrl}/__wae/action/greet ，JSON body；非 2xx 抛错
```

自定义 `fetch`：`createClient({ server: { baseUrl, fetchImpl } })`。

服务端错误：HTTP 层看 `Response.ok` / status；`action` 在非 ok 时抛 `Error`。协议级 `WaeError` 见 `@wae/types` / `@wae/protocol`（经 bridge 的路径）。

## 浏览器 vs WebView

| | 浏览器 | WebView / native |
|--|--------|------------------|
| 默认 bridge | `createBrowserBridge` | `createNativeIpcBridge`（探测到 native 时） |
| 原生能力 | 无 | 经 host；前端不可默认可信 |
| 手动注入 | `bridge: createBrowserBridge()` | 自备 `postMessage` / `onMessage` 的 transport |

## adapter 负责什么

adapter 只负责把**已创建的** `WaeClient` 放进框架上下文。本包不负责 React hooks 或 Vue inject。

## 公开导出（摘要）

值：`createClient`、`createServerClient`、`createBrowserBridge`、`createNativeIpcBridge`、`createBrowserNavigation`、`createLifecycle`、`createMemorySession`、`createWebSocketClient`、`detectEnvironment`。

`createRuntime` 为 `createClient` 的弃用别名。

## 相关

- 区说明：[`../readme.md`](../readme.md)
- adapters：[`../adapters/readme.md`](../adapters/readme.md)
- server：[`../../backend/server/readme.md`](../../backend/server/readme.md)
