# @wae-example/frontend-webview

## What this example demonstrates

Emphasizes “frontend running in WebView”: shares `@wae/client` with pure browser pages but often connects to host bridge
later. Still construct-only skeleton today.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/frontend-webview run check
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
WebView document
  → createClient
  → postMessage / bridge (target)
  → host
  → HostMessage
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Treat this as “page side”; `minimal/bridge-only` / `native/ipc` as “bridge and host side”.
- At type level check whether `client.native` is usable without `hasNativeBridge` (per actual types).
- Do not treat this as an embedded WebView control demo.

## Differences from production apps

Production WebView is provided by desktop/mobile shell; this example has no window or transport.

Dependencies (this example): `@wae/client`.
