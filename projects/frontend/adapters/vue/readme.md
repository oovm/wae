# `@wae/adapter-vue`

把 `@wae/client` 注入 **Vue** 应用（`provide` / `inject` 方向）。不提供组件库；业务 UI 用你自己的 SFC。

## 安装

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-vue@0.0.0
pnpm add vue   # peer
```

## 提供什么

| 导出 | 角色 |
|------|------|
| `vue()` | 配置工厂 `{ name: "vue" }`，给 `defineConfig` |
| `provideWae(app, client)` | 在 Vue 应用上 `provide` 内部 Symbol |
| `useWae()` | 预期 `inject` 同一 client |

Vue 用 **应用级 provide**，不是 React 式 JSX Provider 树——入口通常在 `createApp` 之后立刻 `provideWae`。

## 最小结构（接线完成后）

以下展示调用关系；**0.0.0 中 `useWae` 抛错**，`provideWae` 仅写入 Symbol，尚未接官方 `inject`。

```ts
import { createApp } from "vue";
import { createClient } from "@wae/client";
import vue, { provideWae, useWae } from "@wae/adapter-vue";

const client = createClient({ server: { baseUrl: "/api" } });
const app = createApp({
  setup() {
    // const wae = useWae();
    return {};
  },
});
provideWae(app, client);
app.mount("#app");

export default vue;
```

## SSR / hydration / 响应式

| 能力 | 0.0.0 |
|------|-------|
| SSR / Nuxt | 未实现 |
| hydration | 未实现 |
| 与 `ref` / `reactive` 同步 | 未实现；请求结果请自行写入 ref |
| 组合式 API | 目标形态；当前 `useWae` 未接 `inject` |

## 相关

- [`@wae/client`](../../runtime/readme.md)
- 示例：[`examples/integration/vue`](../../../examples/integration/vue/readme.md)
