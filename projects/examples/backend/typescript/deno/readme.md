# @wae-example/backend-ts-deno

## What this example demonstrates

Deno deployment surface: depends on `@wae/server-deno`, target is export adapted `fetch`. Source still only constructs
`createServer()`.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.
- Repo daily uses Node 22 for `tsc`; real `deno run` needs Deno installed; no Deno entry file in this example yet.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/backend-ts-deno run check
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
- `package.json` — includes `@wae/server-deno`

## Request / event path (target semantics)

```text
createServer
  → @wae/server-deno adapter
  → Deno.serve / export fetch(request)
  → Response
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Test routes with `app.fetch` on Node side first (types still via this package `check`).
- Read `@wae/server-deno` exports; draft minimal `export default { fetch }` in a temp file.
- Do not assume this directory has `deno.json`.

## Differences from production apps

Production runs on Deno Deploy / self-hosted Deno; this example does not start Deno.

Dependencies (this example): `@wae/server`, `@wae/serverless`, `@wae/server-deno`.
