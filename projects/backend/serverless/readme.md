# `@wae/serverless`

把 `@wae/server` 的 `WaeServerApp` 适配成 **标准 Fetch 导出** `{ fetch }`。适合 Worker / Edge / 任何「导出 fetch」的宿主；不是另一套业务 API。

## 安装

```bash
pnpm add @wae/server@0.0.0 @wae/serverless@0.0.0
```

## 生命周期差异

| | `@wae/server` | `@wae/serverless` |
|--|---------------|-------------------|
| 入口 | `app.fetch(request, context?)` | `adaptFetch(app).fetch(request, env, ctx?)` |
| env | 由调用方塞进 context | 第二参数 `env` 传入 |
| `waitUntil` | `execution.waitUntil` | `ctx.waitUntil` |

serverless **没有**长驻进程句柄；每次请求独立。长驻请用 `@wae/server-node`。

## 用法

```ts
import { createServer, route } from "@wae/server";
import { adaptFetch } from "@wae/serverless";

const app = createServer({
  routes: [route("GET", "/", (ctx) => ctx.text("ok"))],
});

export const { fetch } = adaptFetch(app);
// fetch(request, env, { waitUntil })
```

`serverless` 是 `adaptFetch` 的 deprecated 别名，新代码请用 `adaptFetch`。

## 其它导出

| API | 用途 |
|-----|------|
| `readBinding(map, name)` | 从 binding 映射取值 |
| `runWithLifecycle(hooks, fn)` | 包装启动 / 关闭钩子（骨架辅助） |

## 相关

- Cloudflare：[`@wae/server-cloudflare`](../adapters/cloudflare/readme.md)
- Deno：[`@wae/server-deno`](../adapters/deno/readme.md)
