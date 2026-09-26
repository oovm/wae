# `@wae/adapter-solid`

Injects `@wae/client` into a **Solid** tree. Solid uses fine-grained reactivity; **do not** assume React re-render
model. This package only plans Provider / `useWae` boundary.

## Install

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-solid@0.0.0
pnpm add solid-js   # peer
```

## What it provides

| Export        | Role                                      |
|---------------|-------------------------------------------|
| `solid()`     | `{ name: "solid" }` for `defineConfig`    |
| `WaeProvider` | Expected Solid `createContext` for client |
| `useWae()`    | Read `WaeClient` in subtree               |

Same names as React adapter **different implementation**: Solid JSX and `createSignal` lifecycle differ; do not mix
`@wae/adapter-react`.

## Minimal structure (when wired)

Shows call relationships; **in 0.0.0 `WaeProvider` returns `null` and `useWae` throws**.

```tsx
import { createClient } from "@wae/client";
import solid, { WaeProvider, useWae } from "@wae/adapter-solid";

const client = createClient({ server: { baseUrl: "/api" } });

function Ping() {
  const wae = useWae();
  return <button type="button" onClick={() => wae.server.fetch("/hello")}>ping</button>;
}

export function App() {
  return (
    <WaeProvider client={client}>
      <Ping />
    </WaeProvider>
  );
}

export default solid;
```

## SSR / hydration / reactivity

| Capability            | 0.0.0                            |
|-----------------------|----------------------------------|
| SolidStart SSR        | Not implemented                  |
| hydration             | Not implemented                  |
| With `createResource` | Not wrapped; wrap async yourself |

## Related

- [`@wae/client`](../client/readme.md)
- Example: [`examples/integration/solid`](../../examples/integration/solid/readme.md)
