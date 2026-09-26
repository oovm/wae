# @wae-example/fullstack-realtime

## What this example demonstrates

Realtime sync intent (subscribe / push / multi-client consistency), more application-layer than “single WebSocket echo”.
Still client+server construction only.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/fullstack-realtime run check
```

From this directory:

```bash
pnpm run check
pnpm run build   # currently prints skeleton, no deployable output
```

## Access URL

**None.** No localhost port or openable static page. Today’s acceptance is only `pnpm run check` (`tsc --noEmit`).

## Key files

- `src/main.ts`
- `package.json`

## Request / event path (target semantics)

```text
client subscribes to interest set
  → server accepts (HTTP or WS)
  → event bus / storage change
  → push → multiple clients update
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Simulate a “room” with in-memory Map; use repeated `app.fetch` as command channel before push exists.
- Versus `fullstack/websocket`: this directory practices sync model, that one transport.
- Do not import third-party realtime SaaS as built-in WAE capability.

## Differences from production apps

Production has persistence, fan-out, backpressure; this example has no event bus.

Dependencies (this example): `@wae/client`, `@wae/server`, `@wae/serverless`.
