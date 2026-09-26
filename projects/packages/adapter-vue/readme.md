# `@wae/adapter-vue`

Injects `@wae/client` into **Vue** apps (`provide` / `inject`). No component library; use your own SFCs for UI.

## Install

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-vue@0.0.0
pnpm add vue   # peer ^3.5
```

## What it provides

| Export                    | Role                                              |
|---------------------------|---------------------------------------------------|
| `vue()`                   | `{ name: "vue" }` for `defineConfig`              |
| `provideWae(app, client)` | `app.provide` with internal Symbol                |
| `useWae()`                | `inject` same `WaeClient`; throws if not provided |

## Minimal usage

```ts
import { createApp } from "vue";
import { createClient } from "@wae/client";
import vue, { provideWae } from "@wae/adapter-vue";
import App from "./App.vue";

const client = createClient({ server: { baseUrl: "/api" } });
const app = createApp(App);
provideWae(app, client);
app.mount("#app");

export default vue;
```

Call `useWae()` in child components / SFC `setup`.

## SSR / hydration / reactivity

| Capability      | Status                                    |
|-----------------|-------------------------------------------|
| SSR / Nuxt      | Not implemented                           |
| hydration       | Not implemented                           |
| Sync with `ref` | Write request results into `ref` yourself |

## Related

- [`@wae/client`](../client/readme.md)
- Runnable example: [`examples/integration/vue-app`](../../examples/integration/vue-app/readme.md)
