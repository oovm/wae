# `@wae/server-deno`

Connects `@wae/server` to **Deno** request entry: returns `(request) => Response`, via `@wae/serverless` `adaptFetch`.

## Install

```bash
pnpm add @wae/server@0.0.0 @wae/server-deno@0.0.0
# Or resolve npm:@wae/server-deno@0.0.0 in a Deno project per your package manager
```

## Entry

```ts
import { createServer, route } from "@wae/server";
import { serve } from "@wae/server-deno";

const app = createServer({
  routes: [route("GET", "/", (ctx) => ctx.text("deno"))],
});

const handler = serve(app);
export default { fetch: handler };
// or: Deno.serve(handler)
```

`serve(app)` returns `(request: Request) => Promise<Response>`; `env` is currently an empty object.

## Environment and limits

| Item         | Notes                                                                |
|--------------|----------------------------------------------------------------------|
| Deno version | Not pinned in repo CI; verify on recent Deno 1.x / 2.x               |
| Filesystem   | Deno permission model (`--allow-read` etc.); do not assume Node `fs` |
| WebSocket    | Use Deno native APIs; not wrapped here                               |
| Timers       | Available; semantics differ from Worker `waitUntil`                  |

Deploy via your own `deno run` / Deploy config; this package only provides a handler factory.

## Related

- [`@wae/serverless`](../serverless/readme.md)
- Example: [`examples/backend/typescript/deno`](../../examples/backend/typescript/deno/readme.md)
