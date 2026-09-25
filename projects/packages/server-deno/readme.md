# `@wae/server-deno`

把 `@wae/server` 接到 **Deno** 的请求入口：返回 `(request) => Response`，底层经 `@wae/serverless` 的 `adaptFetch`。

## 安装

```bash
pnpm add @wae/server@0.0.0 @wae/server-deno@0.0.0
# 或在 Deno 工程中按你的包管理方式解析 npm:@wae/server-deno@0.0.0
```

## 入口

```ts
import { createServer, route } from "@wae/server";
import { serve } from "@wae/server-deno";

const app = createServer({
  routes: [route("GET", "/", (ctx) => ctx.text("deno"))],
});

const handler = serve(app);
export default { fetch: handler };
// 或：Deno.serve(handler)
```

`serve(app)` 返回的函数签名为 `(request: Request) => Promise<Response>`；`env` 目前传入空对象。

## 环境与限制

| 项 | 说明 |
|----|------|
| Deno 版本 | 未在本仓 CI 固定；请用较新的 Deno 1.x / 2.x 自行验证 |
| 文件系统 | 走 Deno 权限模型（`--allow-read` 等），不要假设 Node `fs` |
| WebSocket | 用 Deno 原生 API；本包未封装 |
| 定时器 | 可用；与 Worker 的 `waitUntil` 语义不同 |

部署：由你自己的 `deno run` / Deploy 配置完成；本包只提供 handler 工厂。

## 相关

- [`@wae/serverless`](../serverless/readme.md)
- 示例：[`examples/backend/typescript/deno`](../../examples/backend/typescript/deno/readme.md)
