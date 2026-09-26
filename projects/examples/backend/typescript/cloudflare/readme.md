# @wae-example/backend-ts-cloudflare

## What this example demonstrates

Cloudflare Workers surface: depends on `@wae/server-cloudflare` (`createCloudflareApp` / `createWorker` APIs). Source
does not call those factories yet.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.
- No `wrangler.toml`, no `wrangler dev` script; do not invent wired Workers flow.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/backend-ts-cloudflare run check
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
- `package.json` — includes `@wae/server-cloudflare`

## Request / event path (target semantics)

```text
createServer
  → createCloudflareApp / adapt
  → Worker fetch(request, env, ctx)
  → Response
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Verify platform-agnostic handlers with `route` + `app.fetch`.
- Read `@wae/server-cloudflare` README; match Worker signature and `env` bindings.
- Versus `backend/typescript/fetch`: that one is generic serverless, this pins CF.

## Differences from production apps

Production uses Wrangler publish and KV/R2 bindings; this example has no Worker bundle.

Dependencies (this example): `@wae/server`, `@wae/serverless`, `@wae/server-cloudflare`.
