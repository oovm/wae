# `@wae/serverless`

Adapts `@wae/server` `WaeServerApp` to a **standard Fetch export** `{ fetch }`. For Worker / Edge / any host that
exports `fetch`; not a separate business API.

## Install

```bash
pnpm add @wae/server@0.0.0 @wae/serverless@0.0.0
```

## Lifecycle differences

|             | `@wae/server`                  | `@wae/serverless`                           |
|-------------|--------------------------------|---------------------------------------------|
| Entry       | `app.fetch(request, context?)` | `adaptFetch(app).fetch(request, env, ctx?)` |
| env         | Caller puts in context         | Second param `env`                          |
| `waitUntil` | `execution.waitUntil`          | `ctx.waitUntil`                             |

serverless has **no** long-lived process handle; each request is independent. For long-lived use `@wae/server-node`.

## Usage

```ts
import { createServer, route } from "@wae/server";
import { adaptFetch } from "@wae/serverless";

const app = createServer({
  routes: [route("GET", "/", (ctx) => ctx.text("ok"))],
});

export const { fetch } = adaptFetch(app);
// fetch(request, env, { waitUntil })
```

`serverless` is a deprecated alias for `adaptFetch`; new code should use `adaptFetch`.

## Other exports

| API                           | Purpose                                         |
|-------------------------------|-------------------------------------------------|
| `readBinding(map, name)`      | Read from binding map                           |
| `runWithLifecycle(hooks, fn)` | Wrap startup / shutdown hooks (skeleton helper) |

## Related

- Cloudflare: [`@wae/server-cloudflare`](../adapters/cloudflare/readme.md)
- Deno: [`@wae/server-deno`](../adapters/deno/readme.md)
