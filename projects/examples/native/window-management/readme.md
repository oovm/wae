# @wae-example/native-window-management

## What this example demonstrates

Window management intent: minimize, multi-window, sizing handled by host/shell. Currently only enables native bridge
flag.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/native-window-management run check
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
Frontend UI action
  → window.* ClientMessage (target)
  → bridge → host
  → shell adjusts OS window
  → optional HostMessage ack
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Read with `native/desktop-shell`: shell owns window lifecycle, this directory focuses window control messages.
- Passing `check` is today’s acceptance; no fake window screenshots.
- Keep path bridge→host.

## Differences from production apps

Production has multi-monitor, fullscreen, platform differences; this example has no window.

Dependencies (this example): `@wae/client`.
