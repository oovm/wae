# backend

给**服务端开发者**：用 `@wae/server` 写平台无关 handler，再用 `serverless` / `server-*` 接到具体运行时。

本地 **Rust host** 不是这里的「后端」——host 管 WebView 壳与 IPC；远程业务或 Worker 才走本目录。

## 分层

```text
@wae/server          路由 + fetch 应用
       ↓
@wae/serverless      adaptFetch → Worker 风格入口
       ↓
@wae/server-node | server-deno | server-cloudflare
```

写业务只依赖 `@wae/server`。换部署目标时换 adapter，不要改 handler 签名。

## 目录

| 路径 | npm | 说明 |
|------|-----|------|
| [`server/`](server/readme.md) | `@wae/server` | `createServer` / `route` |
| [`serverless/`](serverless/readme.md) | `@wae/serverless` | `adaptFetch` |
| [`adapters/node/`](adapters/node/readme.md) | `@wae/server-node` | 长驻进程（骨架） |
| [`adapters/deno/`](adapters/deno/readme.md) | `@wae/server-deno` | Deno `fetch` 适配 |
| [`adapters/cloudflare/`](adapters/cloudflare/readme.md) | `@wae/server-cloudflare` | Workers / KV / DO 入口 |

## 一次请求路径

```text
HTTP Request
  → adapter 注入 env / execution
  → app.fetch
  → middleware → 匹配 route
  → handler(ctx) → ctx.json / ctx.text / Response
```

未匹配路由返回 `404`；handler 抛错返回 `500` 与错误消息文本。
