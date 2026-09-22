# `@wae/adapter-vue`

把 `@wae/client` 注入 **Vue** 应用（`provide` / `inject`）。不提供组件库；业务 UI 用你自己的 SFC。

## 安装

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-vue@0.0.0
pnpm add vue   # peer ^3.5
```

## 提供什么

| 导出 | 角色 |
|------|------|
| `vue()` | `{ name: "vue" }`，给 `defineConfig` |
| `provideWae(app, client)` | `app.provide` 内部 Symbol |
| `useWae()` | `inject` 同一 `WaeClient`；未 provide 时抛错 |

## 最小用法

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

在子组件 / SFC 的 `setup` 中调用 `useWae()`。

## SSR / hydration / 响应式

| 能力 | 现状 |
|------|------|
| SSR / Nuxt | 未实现 |
| hydration | 未实现 |
| 与 `ref` 同步 | 自行把请求结果写入 `ref` |

## 相关

- [`@wae/client`](../../runtime/readme.md)
- 可跑示例：[`examples/integration/vue-app`](../../../examples/integration/vue-app/readme.md)
