# `@wae/adapter-svelte`

把 `@wae/client` 放进 **Svelte** context（`setContext` / `getContext` 方向）。不提供 `.svelte` UI 组件包。

## 安装

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-svelte@0.0.0
pnpm add svelte   # peer
```

## 提供什么

| 导出 | 角色 |
|------|------|
| `svelte()` | `{ name: "svelte" }`，给 `defineConfig` |
| `setWaeContext(client)` | 预期在根组件 `onMount`/初始化时写入 context |
| `getWaeContext()` | 子组件读取 `WaeClient` |

Svelte 的 context **不能跨组件树随意提升**；必须在父组件设置后，子组件才能 `get`。这与 React Provider 包裹、Vue 应用级 provide 都不同。

## 最小结构（接线完成后）

以下展示调用关系；**0.0.0 中 `setWaeContext` 为空实现，`getWaeContext` 抛错**。

```svelte
<!-- App.svelte（示意） -->
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

## SSR / hydration / 响应式

| 能力 | 0.0.0 |
|------|-------|
| SvelteKit SSR | 未实现 |
| hydration | 未实现 |
| 与 `$state` / stores | 未绑定；请自行把请求结果写入 store |

## 相关

- [`@wae/client`](../../runtime/readme.md)
- 示例：[`examples/integration/svelte`](../../../examples/integration/svelte/readme.md)
