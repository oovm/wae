# @wae-example/integration-vue

## What this example demonstrates

Vue injection boundary: depends on `@wae/client` + `@wae/adapter-vue`. Formal injection is **app-level**
`provideWae(app, client)`, not React-style JSX Provider tree. This directory’s `src/main.ts` only constructs client and
imports adapter; **no** `createApp` / `.vue` files.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10
- peer: `vue` (install yourself; no runtime mount in this example)
- 0.0.0: `useWae` may still throw “not provided”—expected skeleton

## Startup commands

```bash
pnpm --filter @wae-example/integration-vue run check
```

## Access URL

**None.** Success: `tsc --noEmit` passes.

## Key files

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-vue"`
- Package docs: [`@wae/adapter-vue`](../../../frontend/adapters/vue/readme.md)

## Request / event path

```text
createApp(...)
  → provideWae(app, client)     ← Vue-specific entry
  → useWae() in setup()
  → client.server.fetch / action → HTTP → remote server
```

## Exercises

- Open `@wae/adapter-vue` README: exports are `provideWae` / `useWae` / `vue()`, not `WaeProvider`.
- In your Vue project: `createClient` then `provideWae`; do not reimplement fetch in adapter.
- Sync with Composition API `ref` is your job; no reactive wrapper in this package.

## Differences from production apps

Production has SFCs, Vite/Vue plugins, routing; adapter SSR / Nuxt not wired. No components or pages here.

Dependencies: `@wae/client`, `@wae/adapter-vue`.
