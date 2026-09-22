# `@wae/server`

平台无关的 Fetch 应用核心：路由、中间件、`ctx.json` / `ctx.text`。不绑定 Node、Deno 或 Cloudflare；接到运行时用 `@wae/serverless` 或 `@wae/server-*`。

## 安装

```bash
pnpm add @wae/server@0.0.0
```

## 一次真实请求路径

```ts
import { createServer, route } from "@wae/server";

const app = createServer({
  routes: [
    route("GET", "/hello/:name", (ctx) => {
      return ctx.json({ hello: ctx.request.params.name });
    }),
  ],
  middleware: [
    async (ctx, next) => {
      const res = await next();
      res.headers.set("x-wae", "1");
      return res;
    },
  ],
});

const res = await app.fetch(new Request("http://local/hello/world"));
// → 200 JSON { "hello": "world" }
```

```text
Request
  → 解析 method / pathname
  → 依次 middleware（可调用 next）
  → 匹配 route（支持 :param）
  → handler(ctx)
  → Response
未匹配 → 404 "Not Found"
handler 抛错 → 500 + 错误消息文本
```

## `ctx` 能力

| 字段/方法 | 含义 |
|-----------|------|
| `request` | `method` / `url` / `headers` / `raw` / `params` |
| `env` / `services` | 由调用方或 `services` 工厂注入 |
| `signal` | AbortSignal |
| `waitUntil` | 转给 execution（Worker 风格） |
| `json` / `text` | 快捷 Response |
| `platform` | 可选平台附加对象 |

`route(method, path, handler)` 或 `route(path, handler)`（method 为 `*`）。

## 与 serverless / 运行时

```ts
import { adaptFetch } from "@wae/serverless";
export default adaptFetch(app); // { fetch }

// 或
import { createCloudflareApp } from "@wae/server-cloudflare";
export default createCloudflareApp(app);
```

## 当前限制

- 无内置 WebSocket 升级、流式 body 助手、文件上传解析。
- 无鉴权中间件内置实现。
- 路由为简单段匹配，无正则/通配尾段。

## 相关

- [`../serverless/readme.md`](../serverless/readme.md)
- [`../adapters/node/readme.md`](../adapters/node/readme.md)
- [`../adapters/deno/readme.md`](../adapters/deno/readme.md)
- [`../adapters/cloudflare/readme.md`](../adapters/cloudflare/readme.md)
