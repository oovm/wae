# @wae-example/integration-svelte-app

## What this example demonstrates

**Desktop WebView + Svelte 5**: `target: "desktop"` → `wae run` starts Vite first, then launches `wae-desktop`.

## Prerequisites

- Repo root has `pnpm install`
- `pnpm --filter @wae/wae run build`
- `pnpm --filter @wae/adapter-svelte run build`
- `cargo build -p wae-desktop` (first time)
- Windows: WebView2 Runtime installed

## Startup commands

```bash
pnpm --filter @wae-example/integration-svelte-app exec wae run --port 5175
```

Browser only: `wae run --platform web`.

## Key files

- `wae.config.ts` — `framework: "svelte"` + desktop
- `src/App.svelte` · `src/styles.css`
- Host: `projects/crates/wae-desktop`

Dependencies: `@wae/client` · `@wae/adapter-svelte` · `svelte` · Vite.
