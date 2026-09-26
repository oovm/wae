# @wae-example/integration-vue-app

## What this example demonstrates

**Desktop WebView + Vue**: `target: "desktop"` → `wae run` starts Vite first, then launches `wae-desktop` native window
loading that page.

## Prerequisites

- Repo root has `pnpm install`
- `pnpm --filter @wae/wae run build`
- `cargo build -p wae-desktop` (first time)
- Windows: WebView2 Runtime installed
- Node.js 22 + pnpm 10

## Startup commands

```bash
# From repo root or with cwd in this package
pnpm --filter @wae-example/integration-vue-app exec wae run --port 5173
```

Default reads `wae.config.ts` `target: "desktop"` / `platform.client: "win32-x64"`.  
Browser-only try: `wae run --platform web`.

## Access URL

Native window title "WAE · vue", same content as Vite page ("WAE + Vue" / ping). **Not** the system default browser tab.

## Key files

- `wae.config.ts` — `target: "desktop"`
- `src/App.vue` · `src/main.ts`
- Host: `projects/crates/wae-desktop` (`wae-desktop`)

## Request / event path

```text
wae run
  → Vite (frontend)
  → @wae/wae-win32-x64.run({ url })
  → cargo/binary wae-desktop --url …
  → WebView2 window
```

## Exercises

- Closing window should also stop Vite.
- Change `target: "web"` to compare browser path.

## Differences from production apps

Production bundles prebuilt static assets into shell; this example dev-bypasses Vite URL.

Dependencies: `@wae/client` · `@wae/adapter-vue` · `vue` · `@wae/wae` · `vite` · local `wae-desktop`.
