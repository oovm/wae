# `@wae/adapter-react`

Injects an existing `@wae/client` into a **React** tree. Provider / hook only; **no** UI components.

## Install

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-react@0.0.0
pnpm add react react-dom   # peer ^19
```

## What it provides

| Export        | Role                                                 |
|---------------|------------------------------------------------------|
| `react()`     | `{ name: "react" }` for `defineConfig`               |
| `WaeProvider` | Context Provider with `client` + `children`          |
| `useWae()`    | Read `WaeClient` in subtree; throws without Provider |

## Minimal usage

```tsx
import { createClient } from "@wae/client";
import react, { WaeProvider, useWae } from "@wae/adapter-react";

const client = createClient({ server: { baseUrl: "/api" } });

function Greeter() {
  const wae = useWae();
  return <button type="button" onClick={() => void wae.server.fetch("/hello")}>ping</button>;
}

export function App() {
  return (
    <WaeProvider client={client}>
      <Greeter />
    </WaeProvider>
  );
}

export default react;
```

## SSR / hydration / state

| Capability      | Status               |
|-----------------|----------------------|
| SSR             | Not implemented      |
| hydration       | Not implemented      |
| Error / loading | Handle in components |

## Related

- [`@wae/client`](../client/readme.md)
- Runnable example: [`examples/integration/react-app`](../../examples/integration/react-app/readme.md)
