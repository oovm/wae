# `@wae/client`

Framework-agnostic frontend runtime. Use directly for plain TS / DOM; Vue / React / Svelte / Solid inject via separate
`@wae/adapter-*`. **Do not** look here for components or hooks.

## Install

```bash
pnpm add @wae/client@0.0.0
```

## What `createClient` creates

```ts
import { createClient } from "@wae/client";

const client = createClient({
  server: { baseUrl: "/api" }, // required
});

// client.env          runtime environment detection
// client.server       HTTP / action client
// client.native.bridge browser or native IPC bridge
// client.session      default in-memory session
// client.lifecycle    app lifecycle hooks
// client.navigation   default browser navigation
// client.connectWs(url) → WebSocket client
```

Optional overrides: `env`, `session`, `lifecycle`, `navigation`, `bridge`.

## How to send requests

```ts
const res = await client.server.fetch("/hello");
const data = await res.json();

const greet = client.server.action<{ name: string }, { ok: boolean }>("greet");
const out = await greet.execute({ name: "wae" });
// POST {baseUrl}/__wae/action/greet with JSON body; throws on non-2xx
```

Custom `fetch`: `createClient({ server: { baseUrl, fetchImpl } })`.

Server errors: HTTP layer uses `Response.ok` / status; `action` throws `Error` when not ok. Protocol-level `WaeError`
see `@wae/types` / `@wae/protocol` (bridge path).

## Browser vs WebView

|                     | Browser                         | WebView / native                                  |
|---------------------|---------------------------------|---------------------------------------------------|
| Default bridge      | `createBrowserBridge`           | `createNativeIpcBridge` (when native detected)    |
| Native capabilities | None                            | Via host; frontend must not be trusted by default |
| Manual injection    | `bridge: createBrowserBridge()` | Supply transport with `postMessage` / `onMessage` |

## What adapters do

Adapters only put an **already created** `WaeClient` into framework context. This package does not provide React hooks
or Vue inject.

## Public exports (summary)

Values: `createClient`, `createServerClient`, `createBrowserBridge`, `createNativeIpcBridge`, `createBrowserNavigation`,
`createLifecycle`, `createMemorySession`, `createWebSocketClient`, `detectEnvironment`.

`createRuntime` is a deprecated alias for `createClient`.

## Related

- Area overview: [`../readme.md`](../readme.md)
- Adapters: [`../adapters/readme.md`](../adapters/readme.md)
- Server: [`../server/readme.md`](../server/readme.md)
