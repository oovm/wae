# @wae-example/integration-svelte

## 这个示例展示什么

Svelte 注入边界：依赖 `@wae/client` + `@wae/adapter-svelte`。正式注入是 **`setWaeContext` / `getWaeContext`**（Svelte context），必须在父组件设置后子组件才能读——不能像 Vue 那样在 `createApp` 之后全局 provide 一次就完事。本目录无 `.svelte` 文件。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10
- peer：`svelte` 5.x（自行安装）
- 0.0.0：`setWaeContext` 可能为空实现，`getWaeContext` 抛错——属骨架预期

## 启动命令

```bash
pnpm --filter @wae-example/integration-svelte run check
```

## 访问地址

**无。** 成功标准：`tsc --noEmit` 通过。

## 关键文件

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-svelte"`
- 包说明：[`@wae/adapter-svelte`](../../../frontend/adapters/svelte/readme.md)

## 请求 / 事件路径

```text
createClient
  → 根组件 setWaeContext(client)   ← Svelte 特有入口
  → 子组件 getWaeContext()
  → client.server.fetch / action → HTTP → 远程 server
```

## 练习点

- 打开 `@wae/adapter-svelte` README：没有 `WaeProvider`，也没有 Vue 的 `provideWae(app, …)`。
- context **不能跨组件树随意提升**；放错层级会读不到。
- 与 `$state` / stores 的同步请自行绑定；SvelteKit SSR 未接线。

## 与生产应用的差异

生产有 `.svelte` + Vite 插件；本示例无组件、无页面。

依赖：`@wae/client`、`@wae/adapter-svelte`。
