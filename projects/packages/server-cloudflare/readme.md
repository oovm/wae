# `@wae/server-cloudflare`

把 `@wae/server` 接到 **Cloudflare Workers**（及兼容的 `fetch(request, env, ctx)` 形态）。经 `@wae/serverless` 注入 `env` 与 `waitUntil`。

## 安装

```bash
pnpm add @wae/server@0.0.0 @wae/serverless@0.0.0 @wae/server-cloudflare@0.0.0
```

## 入口

```ts
import { createServer, route } from "@wae/server";
import { createCloudflareApp, cloudflareKv } from "@wae/server-cloudflare";

const app = createServer({
  routes: [
    route("GET", "/", (ctx) => ctx.json({ hello: "worker" })),
  ],
});

export default createCloudflareApp(app);
// 等价：createWorker(app)
```

Worker 收到请求后：

```text
fetch(request, env, ctx)
  → adaptFetch
  → app.fetch（env / waitUntil 进入 WaeContext）
  → handler → Response
```

## KV 辅助

```ts
const kv = cloudflareKv(env.MY_KV);
await kv.put("k", JSON.stringify({ v: 1 }));
const row = await kv.get<{ v: number }>("k");
```

`get` 会尝试 `JSON.parse`；失败则当字符串返回。

## 子路径导出

| 导出 | 文件 | 用途 |
|------|------|------|
| `@wae/server-cloudflare` | `index` | Worker 主入口 |
| `@wae/server-cloudflare/durable-objects` | DO 适配骨架 | 见包内类型 |
| `@wae/server-cloudflare/queues` | Queue 适配骨架 | 见包内类型 |

## 环境与限制

| 项 | 说明 |
|----|------|
| 运行时 | Cloudflare Workers；本地可用 `wrangler dev`（需自备 `wrangler.toml`） |
| 文件系统 | **无** Node `fs`；用 KV / R2 / D1 等绑定 |
| WebSocket | Workers 有独立 Hibernation API；本包未封装 |
| `waitUntil` | 经 `ctx.waitUntil` 传入 server context |

部署命令取决于你的 Wrangler 工程，例如 `wrangler deploy`。本包不附带 wrangler 配置。

## 相关

- [`@wae/serverless`](../serverless/readme.md)
- 示例：[`examples/backend/typescript/cloudflare`](../../examples/backend/typescript/cloudflare/readme.md)
