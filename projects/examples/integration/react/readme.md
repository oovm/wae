# @wae-example/integration-react

## What this example demonstrates

React injection boundary: depends on `@wae/client` + `@wae/adapter-react`. Formal injection is JSX **`WaeProvider`** +
hook **`useWae`**. This directory only constructs client and imports adapter; **no** React tree or `react-dom` mount.

## Prerequisites

- Repo root has `pnpm install`
- Node.js 22 + pnpm 10
- peer: `react` / `react-dom` (React 19 class; install yourself)
- 0.0.0: `WaeProvider` may return `null`, `useWae` throws—expected skeleton

## Startup commands

```bash
pnpm --filter @wae-example/integration-react run check
```

## Access URL

**None.** Success: `tsc --noEmit` passes.

## Key files

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-react"`
- Package docs: [`@wae/adapter-react`](../../../frontend/adapters/react/readme.md)

## Request / event path

```text
createClient
  → <WaeProvider client={client}>   ← React-specific entry
  → useWae() in child components
  → client.server.fetch / action → HTTP → remote server
```

## Exercises

- Open `@wae/adapter-react` README: exports are `WaeProvider` / `useWae` / `react()`.
- **Do not** mix with `@wae/adapter-solid` same-name APIs (different implementation).
- Strict Mode double invoke and concurrent Provider behavior need handling in real implementation; skeleton does not
  cover.
- loading / error: wrap `server.action` with `useState` in components; adapter has none built-in.

## Differences from production apps

Production has full React project; SSR / hydration not implemented. No JSX runtime here.

Dependencies: `@wae/client`, `@wae/adapter-react`.
