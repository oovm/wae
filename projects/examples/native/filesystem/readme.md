# @wae-example/native-filesystem

## What this example demonstrates

Host filesystem access intent: frontend does not call `fs` directly; requests native capability via bridge. Source sends
no FS messages yet.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/native-filesystem run check
```

From this directory:

```bash
pnpm run check
pnpm run build   # currently prints skeleton, no deployable output
```

## Access URL

**None.** No localhost port or openable static page. Today’s acceptance is only `pnpm run check` (`tsc --noEmit`).

## Key files

- `src/main.ts` — desktop + `hasNativeBridge`
- `package.json`

## Request / event path (target semantics)

```text
Frontend requests read/write path
  → ClientMessage
  → bridge → host
  → host FS API
  → HostMessage (content or error)
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Look up file-related message names in protocol / host docs (per repo state).
- Practice constructing client at type level; real FS needs host implementation.
- Do not swap to Node `fs` pretending WAE native—wrong layer.

## Differences from production apps

Production has permission prompts, sandbox paths, user consent; this example has no disk IO.

Dependencies (this example): `@wae/client`.
