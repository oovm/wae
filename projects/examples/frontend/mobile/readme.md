# @wae-example/frontend-mobile

## What this example demonstrates

Mobile shell frontend intent: page runs in Android/iOS WebView. Source is still `createClient` skeleton; mobile env not
set.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.
- Honest status: no mobile simulator flow; no `@wae/wae-android-*` / `ios-*` startup scripts.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/frontend-mobile run check
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
Mobile shell WebView
  → createClient
  → HTTP and/or native bridge → host
  → HostMessage back to page
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Read platform package READMEs: mobile packages are optional distributions, not dependencies of this example.
- Contrast `native/mobile-shell` (IPC/shell side) with this directory (page side).
- Practice `baseUrl` type check only; do not invent `adb`/`xcode` startup commands.

## Differences from production apps

Production needs real mobile shells and store distribution; 0.0.0 platform packages are mostly placeholders.

Dependencies (this example): `@wae/client`.
