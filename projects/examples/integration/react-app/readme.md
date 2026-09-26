# @wae-example/integration-react-app

## What this example demonstrates

Browser-openable **React + `wae run`** project: `defineConfig({ framework: "react" })`, `WaeProvider` / `useWae`, Vite.

## Prerequisites

- Repo root has `pnpm install`
- `pnpm --filter @wae/wae run build`
- Node.js 22 + pnpm 10

## Startup commands

```bash
pnpm exec wae run --port 5174

# Or from repo root
pnpm --filter @wae-example/integration-react-app exec wae run --port 5174
```

cwd must be this directory (contains `wae.config.ts`).

## Access URL

`http://127.0.0.1:5174/` — should show "WAE + React" and ping button.

## Key files

- `wae.config.ts` — `framework: "react"` + `adapter: react()`
- `index.html` / `src/main.tsx`
- `vite.config.ts` — `@vitejs/plugin-react`

## Request / event path

```text
wae run
  → load wae.config.ts
  → Vite (web)
  → createClient + <WaeProvider>
  → useWae() in Panel
```

## Exercises

- Do not mix with `@wae/adapter-vue` provide model.
- Compare placeholder example [`../react`](../react/readme.md).

## Differences from production apps

No SSR, no real HTTP backend; only verifies CLI and React injection.

Dependencies: `@wae/client` · `@wae/adapter-react` · `react` · `@wae/wae` · `vite`.
