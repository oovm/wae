# `@wae/server-node`

Connects `@wae/server` to a **Node.js / Bun long-lived process**. Filesystem, TCP listen, process timers, etc. belong
here (or your own Node code), not in `@wae/server`.

## Install

```bash
pnpm add @wae/server@0.0.0 @wae/server-node@0.0.0
```

## Entry

```ts
import { createServer, route } from "@wae/server";
import { serve } from "@wae/server-node";

const app = createServer({
  routes: [route("GET", "/health", (ctx) => ctx.json({ ok: true }))],
});

const handle = serve(app, { port: 3000, hostname: "127.0.0.1" });
console.log(handle.port, handle.hostname);
await handle.close();
```

| API                    | Description                                          |
|------------------------|------------------------------------------------------|
| `serve(app, options?)` | Returns `ServeHandle`: `port`, `hostname`, `close()` |
| `options.port`         | Default `3000`                                       |
| `options.hostname`     | Default `127.0.0.1`                                  |

## Current limits (0.0.0)

**Does not** open a TCP port or call `node:http` / `Bun.serve`. `serve` only registers config and returns a closable
handle. To verify routes, call directly:

```ts
await app.fetch(new Request("http://127.0.0.1:3000/health"));
```

## Differences from other runtimes

| Capability        | Node (this package)              | Deno / Cloudflare            |
|-------------------|----------------------------------|------------------------------|
| Long-lived listen | Yes (when wired)                 | Usually export fetch         |
| Filesystem        | Node APIs available              | Restricted or different APIs |
| Deploy            | Self-managed process / container | `deno` / `wrangler`          |

## Environment

- Target: Node.js 22+ (matches repo verification); Bun planned compatible, not CI-tested alone.
- Deploy command: when wired, expect something like `node dist/server.js`; **no official startup script today**.

## Related

- Abstraction: [`@wae/server`](../server/readme.md)
- Example: [`examples/backend/typescript/node`](../../examples/backend/typescript/node/readme.md)
