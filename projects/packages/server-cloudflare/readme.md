# `@wae/server-cloudflare`

Connects `@wae/server` to **Cloudflare Workers** (and compatible `fetch(request, env, ctx)` shapes). Injects `env` and
`waitUntil` via `@wae/serverless`.

## Install

```bash
pnpm add @wae/server@0.0.0 @wae/serverless@0.0.0 @wae/server-cloudflare@0.0.0
```

## Entry

```ts
import { createServer, route } from "@wae/server";
import { createCloudflareApp, cloudflareKv } from "@wae/server-cloudflare";

const app = createServer({
  routes: [
    route("GET", "/", (ctx) => ctx.json({ hello: "worker" })),
  ],
});

export default createCloudflareApp(app);
// equivalent: createWorker(app)
```

When the Worker receives a request:

```text
fetch(request, env, ctx)
  → adaptFetch
  → app.fetch (env / waitUntil enter WaeContext)
  → handler → Response
```

## KV helper

```ts
const kv = cloudflareKv(env.MY_KV);
await kv.put("k", JSON.stringify({ v: 1 }));
const row = await kv.get<{ v: number }>("k");
```

`get` tries `JSON.parse`; on failure returns as string.

## Subpath exports

| Export                                   | File                   | Purpose              |
|------------------------------------------|------------------------|----------------------|
| `@wae/server-cloudflare`                 | `index`                | Worker main entry    |
| `@wae/server-cloudflare/durable-objects` | DO adapter skeleton    | See in-package types |
| `@wae/server-cloudflare/queues`          | Queue adapter skeleton | See in-package types |

## Environment and limits

| Item        | Notes                                                                         |
|-------------|-------------------------------------------------------------------------------|
| Runtime     | Cloudflare Workers; local via `wrangler dev` (bring your own `wrangler.toml`) |
| Filesystem  | **No** Node `fs`; use KV / R2 / D1 bindings                                   |
| WebSocket   | Workers have separate Hibernation API; not wrapped here                       |
| `waitUntil` | Passed via `ctx.waitUntil` into server context                                |

Deploy depends on your Wrangler project, e.g. `wrangler deploy`. This package does not ship wrangler config.

## Related

- [`@wae/serverless`](../serverless/readme.md)
- Example: [`examples/backend/typescript/cloudflare`](../../examples/backend/typescript/cloudflare/readme.md)
