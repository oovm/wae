# bridge-only

## What this example demonstrates

Focus on the boundary between frontend bridge and Rust host: messages enter the host, not remote `@wae/server`.

## Prerequisites

This directory **has no** `package.json` / `src`. Testable pieces are crate `wae-bridge`, and `env.hasNativeBridge`
typing in other examples.

## Startup commands

```bash
cargo test -p wae-bridge
cargo check -p wae-bridge
```

Frontend side in a temp file or compare [`native/ipc`](../native/ipc/readme.md):

```ts
import { createClient } from "@wae/client";

const client = createClient({
  server: { baseUrl: "/api" },
  env: { target: "desktop", hasNativeBridge: true },
});
void client.native.bridge;
```

## Access URL

None. Success: `cargo test -p wae-bridge`; frontend today only verifies types and default bridge selection.

## Key files

- This README
- [`host/bridge`](../../../crates/wae-bridge/readme.md)
- Closest example with code: [`native/ipc`](../native/ipc/readme.md)

## Request / event path (target semantics)

```text
ClientMessage
  → MessageTransport / IPC (bridge)
  → host (wae-bridge) handle
  → Option<HostMessage>
  → frontend decode
```

## Exercises

- Separate “remote `@wae/server`” from “local host”—neither is an alias for the other.
- Read `native/ipc` with `hasNativeBridge: true`, then return to this README path diagram.
- In 0.0.0 `handle` may always return `None`—goal is boundary, not feature checklist.

## Differences from production apps

Production needs real WebView `postMessage` transport and platform shell binaries; this directory has no runnable
frontend.

Dependencies: no npm package here. Rust: `wae-bridge`; TS practice: `@wae/client`.
