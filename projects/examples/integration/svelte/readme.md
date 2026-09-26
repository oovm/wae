# @wae-example/integration-svelte

## What this example demonstrates

Svelte injection boundary: depends on `@wae/client` + `@wae/adapter-svelte`. Formal injection is **`setWaeContext` /
`getWaeContext`** (Svelte context); parent must set before children read—unlike Vue global provide after `createApp`. No
`.svelte` files here.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10
- peer: `svelte` 5.x (install yourself)
- 0.0.0: `setWaeContext` may be no-op, `getWaeContext` throws—expected skeleton

## Startup commands

```bash
pnpm --filter @wae-example/integration-svelte run check
```

## Access URL

**None.** Success: `tsc --noEmit` passes.

## Key files

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-svelte"`
- Package docs: [`@wae/adapter-svelte`](../../../frontend/adapters/svelte/readme.md)

## Request / event path

```text
createClient
  → root component setWaeContext(client)   ← Svelte-specific entry
  → child getWaeContext()
  → client.server.fetch / action → HTTP → remote server
```

## Exercises

- Open `@wae/adapter-svelte` README: no `WaeProvider`, no Vue `provideWae(app, …)`.
- Context **cannot be lifted arbitrarily**; wrong hierarchy level reads fail.
- Sync with `$state` / stores yourself; SvelteKit SSR not wired.

## Differences from production apps

Production has `.svelte` + Vite plugin; no components or pages here.

Dependencies: `@wae/client`, `@wae/adapter-svelte`.
