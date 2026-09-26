# @wae-example/frontend-wasm

## What this example demonstrates

Wasm client intent: frontend may explicitly opt into Wasm platform packages. Dependencies are still only `@wae/client`;
**no** `.wasm` loading code.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.
- Wasm is explicit opt-in; do not assume default bundle includes Rust/Wasm.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/frontend-wasm run check
```

From this directory:

```bash
pnpm run check
pnpm run build   # currently prints skeleton, no deployable output
```

## Access URL

**None.** No localhost port or openable static page. Today’s acceptance is only `pnpm run check` (`tsc --noEmit`).

## Key files

- `src/main.ts` — only `createClient`
- `package.json`

## Request / event path (target semantics)

```text
Page / Worker
  → (optional) load @wae/wae-unknown-wasm32 capabilities
  → createClient still uses TS API
  → remote HTTP or other transport
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Read `@wae/wae-unknown-wasm32` README: separate “platform placeholder API” from this example’s client construction.
- Confirm this `package.json` **does not** depend on wasm platform package—intentional minimal surface.
- Keep practice at TS `check`; do not pretend wasm-pack pipeline exists.

## Differences from production apps

Production builds and loads Wasm artifacts; platform API is mostly placeholder; this example does not run Wasm.

Dependencies (this example): `@wae/client`.
