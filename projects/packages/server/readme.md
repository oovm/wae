# `@wae/server`

Platform-agnostic Fetch app core: routing, middleware, `ctx.json` / `ctx.text`. Not bound to Node, Deno, or Cloudflare;
connect to a runtime with `@wae/serverless` or `@wae/server-*`.

## Install

```bash
pnpm add @wae/server@0.0.0
```

## One real request path

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
  → parse method / pathname
  → middleware in order (may call next)
  → match route (supports :param)
  → handler(ctx)
  → Response
No match → 404 "Not Found"
handler throws → 500 + error message text
```

## `ctx` capabilities

| Field/method       | Meaning                                         |
|--------------------|-------------------------------------------------|
| `request`          | `method` / `url` / `headers` / `raw` / `params` |
| `env` / `services` | Injected by caller or `services` factory        |
| `signal`           | AbortSignal                                     |
| `waitUntil`        | Forward to execution (Worker style)             |
| `json` / `text`    | Shortcut Response                               |
| `platform`         | Optional platform attachment                    |

`route(method, path, handler)` or `route(path, handler)` (method is `*`).

## With serverless / runtime

```ts
import { adaptFetch } from "@wae/serverless";
export default adaptFetch(app); // { fetch }

// or
import { createCloudflareApp } from "@wae/server-cloudflare";
export default createCloudflareApp(app);
```

## Current limits

- No built-in WebSocket upgrade, streaming body helpers, or file upload parsing.
- No built-in auth middleware.
- Simple segment routing only; no regex/wildcard tail segments.

## Related

- [`../serverless/readme.md`](../serverless/readme.md)
- [`../adapters/node/readme.md`](../adapters/node/readme.md)
- [`../adapters/deno/readme.md`](../adapters/deno/readme.md)
- [`../adapters/cloudflare/readme.md`](../adapters/cloudflare/readme.md)
