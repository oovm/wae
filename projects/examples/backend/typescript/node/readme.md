# @wae-example/backend-ts-node

## What this example demonstrates

Node backend surface without WAE frontend: depends on `@wae/server` + `@wae/serverless` + `@wae/server-node`.
`src/main.ts` only `createServer()` today; does not call `serve`.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/backend-ts-node run check
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
- `package.json` — includes `@wae/server-node`

## Request / event path (target semantics)

```text
createServer
  → (target) @wae/server-node serve(app)
  → Node HTTP process
  → app.fetch handles Request
(0.0.0 serve is skeleton, may not listen)
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Verify handlers with `route` + `app.fetch` without Node process first.
- Read `@wae/server-node` README whether `serve` still prints skeleton.
- This example **has no** `createClient`—do not copy fullstack exercises.

## Differences from production apps

When wired, expect long-lived Node process and real port; currently no port.

Dependencies (this example): `@wae/server`, `@wae/serverless`, `@wae/server-node`.
