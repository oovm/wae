# @wae-example/backend-ts-fetch

## What this example demonstrates

Plain `fetch` adapter intent: `@wae/server` + `@wae/serverless`, no Node/Deno/CF-specific packages. For any runtime that
can call `app.fetch`.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/backend-ts-fetch run check
```

From this directory:

```bash
pnpm run check
pnpm run build   # currently prints skeleton, no deployable output
```

## Access URL

**None.** No localhost port or openable static page. Today’s acceptance is only `pnpm run check` (`tsc --noEmit`).

## Key files

- `src/main.ts` — `createServer()`
- `package.json` — no `server-node` / `server-deno` / `server-cloudflare`

## Request / event path (target semantics)

```text
External runtime receives Request
  → adaptFetch / app.fetch
  → handler
  → Response
(no listen, no specific cloud API)
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Declare routes and `await app.fetch(request)` as unit test.
- Deliberately do not add `@wae/server-node`; keep portable dependency surface.
- For process or Worker, branch to node/deno/cloudflare examples.

## Differences from production apps

Production still picks a concrete host; this example stops at portable `fetch` boundary.

Dependencies (this example): `@wae/server`, `@wae/serverless`.
