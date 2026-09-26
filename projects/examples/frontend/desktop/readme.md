# @wae-example/frontend-desktop

## What this example demonstrates

Desktop WebView frontend intent: app page side inside a desktop shell. Current `src/main.ts` like browser only
constructs `createClient`; `env.target` not set yet.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10 (matches repo `packageManager`)
- Skeleton: `src/main.ts` only constructs API; no HTTP listen, browser page, or UI.
- Honest status: code does not distinguish desktop env; directory name expresses target shape, not wired shell.

## Startup commands

From repo root:

```bash
pnpm --filter @wae-example/frontend-desktop run check
```

From this directory:

```bash
pnpm run check
pnpm run build   # currently prints skeleton, no deployable output
```

## Access URL

**None.** No localhost port or openable static page. Today’s acceptance is only `pnpm run check` (`tsc --noEmit`).

## Key files

- `src/main.ts` — `createClient` skeleton
- `package.json`

## Request / event path (target semantics)

```text
Desktop shell loads frontend
  → createClient (in page)
  → (optional) HTTP to remote server
  → if native enabled: bridge → host (see native/*)
```

0.0.0 code usually stops at “construct object”; full path not run yet.

## Exercises

- Compare with `frontend/browser`: with only client, desktop adds shell and optional native, not another `createClient`.
- For `hasNativeBridge` see or edit `native/desktop-shell` / `native/ipc`.
- Do not expect `wae dev` to open a window—CLI is still skeleton.

## Differences from production apps

Production depends on `@wae/wae-win32-*` / `darwin-*` / `linux-*` and Rust host binaries; this example does not start a
shell.

Dependencies (this example): `@wae/client`.
