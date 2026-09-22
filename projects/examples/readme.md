# examples — 按用途学习路线

这些示例说明 **WAE 怎么组合**，不是按框架名分桶。框架只出现在 `integration/`。

**总体现状（0.0.0）**：多数示例仍是 API 构造 + `pnpm run check`。**可打开页面**的路径：`integration/vue-app` · `integration/react-app`（`pnpm exec wae run`）。`wae create` / `build` 等仍是骨架。

`minimal/protocol-only` 与 `minimal/bridge-only`、以及 `backend/rust/*` **没有**示例包工程：前者是文档指引，后者仅 README。

## 你想做什么 → 去哪

| 目标 | 目录 |
|------|------|
| 最小 client | [`minimal/client-only`](minimal/client-only/readme.md) |
| 最小 server | [`minimal/server-only`](minimal/server-only/readme.md) |
| 只碰协议编解码 | [`minimal/protocol-only`](minimal/protocol-only/readme.md)（尚无代码） |
| 只碰 bridge | [`minimal/bridge-only`](minimal/bridge-only/readme.md)（尚无代码） |
| 浏览器前端 | [`frontend/browser`](frontend/browser/readme.md) |
| 桌面 / 移动 / WebView / Wasm 前端 | [`frontend/desktop`](frontend/desktop/readme.md) · [`mobile`](frontend/mobile/readme.md) · [`webview`](frontend/webview/readme.md) · [`wasm`](frontend/wasm/readme.md) |
| 全栈 RPC | [`fullstack/rpc`](fullstack/rpc/readme.md) |
| 鉴权 / SSR / WS / 实时 / 上传 | [`fullstack/auth`](fullstack/auth/readme.md) · [`ssr`](fullstack/ssr/readme.md) · [`websocket`](fullstack/websocket/readme.md) · [`realtime`](fullstack/realtime/readme.md) · [`file-upload`](fullstack/file-upload/readme.md) |
| Node / Deno / Cloudflare / 纯 fetch / WS 后端 | [`backend/typescript/*`](backend/typescript/node/readme.md) |
| Rust 服务占位 | [`backend/rust/*`](backend/rust/http/readme.md)（仅文档） |
| 桌面壳 / 移动壳 / IPC / FS / 窗口 | [`native/*`](native/ipc/readme.md) |
| Vue / React / Svelte / Solid 注入边界 | [`integration/vue`](integration/vue/readme.md) 等 |
| **可 `wae run` 的 Vue / React 页** | [`integration/vue-app`](integration/vue-app/) · [`integration/react-app`](integration/react-app/) |

## 推荐顺序

1. `minimal/client-only` + `minimal/server-only` — 分清两层
2. `fullstack/rpc` — 看 client 与 server 如何同目录占位
3. `backend/typescript/node` 或 `cloudflare` — 看部署适配器依赖面
4. `integration/<你的框架>` — 看 adapter 只负责注入，不另起 runtime
5. `native/ipc` — 才进入 bridge → host，而不是 HTTP server

## 目录地图

```text
frontend/      前端运行形态（browser / desktop / mobile / webview / wasm）
fullstack/     client + server 组合意图
backend/       无 WAE 前端的后端（typescript/ · rust/）
native/        壳 · IPC · 系统能力（走 host，不是 createServer）
integration/   框架是变量
minimal/       单能力边界
```

## 公共命令

```bash
# 仓库根：对所有带 package.json 的示例跑 tsc
pnpm --filter "./projects/examples/**" run check
```
