# @wae-example/native-desktop-shell

## What this example demonstrates

Desktop shell intent: page in desktop WebView with native bridge flag enabled. Source matches `native/ipc` shape
(`target: "desktop", hasNativeBridge: true`); directory emphasizes “shell” not single IPC exercise.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/native-desktop-shell run check
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
Desktop shell process
  → loads WebView frontend
  → bridge → host
  → window / system integration (target)
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Versus `frontend/desktop`: that may lack native flag; here explicitly `hasNativeBridge`.
- Read `projects/crates` and `packages/wae-win32-*` platform READMEs for where binaries come from.
- Do not run fictional `wae open-desktop`.

## Differences from production apps

Production has packaged installers and auto-update; this example does not start shell binary.

Dependencies (this example): `@wae/client`.
