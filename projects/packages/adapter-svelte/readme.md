# `@wae/adapter-svelte`

Puts `@wae/client` in **Svelte** context (`setContext` / `getContext` direction). No `.svelte` UI component package.

## Install

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-svelte@0.0.0
pnpm add svelte   # peer
```

## What it provides

| Export                  | Role                                        |
|-------------------------|---------------------------------------------|
| `svelte()`              | `{ name: "svelte" }` for `defineConfig`     |
| `setWaeContext(client)` | Expected in root component init / `onMount` |
| `getWaeContext()`       | Child reads `WaeClient`                     |

Svelte context **cannot be lifted arbitrarily** across the tree; parent must set before children can `get`. Unlike React
Provider wrap or Vue app-level provide.

## Minimal structure (when wired)

Shows call relationships; **in 0.0.0 `setWaeContext` is no-op and `getWaeContext` throws**.

```svelte
<!-- App.svelte (illustrative) -->
<script lang="ts">
  import { createClient } from "@wae/client";
  import { setWaeContext, getWaeContext } from "@wae/adapter-svelte";
  const client = createClient({ server: { baseUrl: "/api" } });
  setWaeContext(client);
</script>
```

```ts
import svelte from "@wae/adapter-svelte";
export default svelte;
```

## SSR / hydration / reactivity

| Capability             | 0.0.0                                      |
|------------------------|--------------------------------------------|
| SvelteKit SSR          | Not implemented                            |
| hydration              | Not implemented                            |
| With `$state` / stores | Not bound; write results to store yourself |

## Related

- [`@wae/client`](../client/readme.md)
- Example: [`examples/integration/svelte`](../../examples/integration/svelte/readme.md)
