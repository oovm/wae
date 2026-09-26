# @wae-example/integration-solid

## What this example demonstrates

Solid injection boundary: depends on `@wae/client` + `@wae/adapter-solid`. API names include `WaeProvider` / `useWae`
but built on Solid **fine-grained reactivity**—do not apply React re-render mental model; **do not** mix
`@wae/adapter-react`. No Solid JSX mount here.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10
- peer: `solid-js` (install yourself)
- 0.0.0: Provider / hook skeleton (may `null` / throw)

## Startup commands

```bash
pnpm --filter @wae-example/integration-solid run check
```

## Access URL

**None.** Success: `tsc --noEmit` passes.

## Key files

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-solid"`
- Package docs: [`@wae/adapter-solid`](../../../frontend/adapters/solid/readme.md)

## Request / event path

```text
createClient
  → <WaeProvider client={client}>   ← Solid context (not React)
  → useWae()
  → client.server.fetch / action → HTTP → remote server
```

## Exercises

- Open `@wae/adapter-solid` README; wrap async with Solid `createResource` yourself (adapter not wrapped).
- Do not copy React example JSX into Solid and expect same runtime.
- SolidStart SSR / hydration not implemented.

## Differences from production apps

Production has Solid build chain; no components or pages here.

Dependencies: `@wae/client`, `@wae/adapter-solid`.
