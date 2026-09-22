# @wae-example/integration-vue

## 这个示例展示什么

Vue 注入边界：依赖 `@wae/client` + `@wae/adapter-vue`。正式注入是**应用级** `provideWae(app, client)`，不是 React 式 JSX Provider 树。本目录的 `src/main.ts` 只构造 client 并 import adapter，**没有** `createApp` / `.vue` 文件。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10
- peer：`vue`（自行安装；本示例未声明运行时挂载）
- 0.0.0：`useWae` 仍可能抛「未提供」——属骨架预期

## 启动命令

```bash
pnpm --filter @wae-example/integration-vue run check
```

## 访问地址

**无。** 成功标准：`tsc --noEmit` 通过。

## 关键文件

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-vue"`
- 包说明：[`@wae/adapter-vue`](../../../frontend/adapters/vue/readme.md)

## 请求 / 事件路径

```text
createApp(...)
  → provideWae(app, client)     ← Vue 特有入口
  → setup() 内 useWae()
  → client.server.fetch / action → HTTP → 远程 server
```

## 练习点

- 打开 `@wae/adapter-vue` README，确认导出是 `provideWae` / `useWae` / `vue()`，不是 `WaeProvider`。
- 自己建 Vue 工程时：先 `createClient`，再 `provideWae`；不要在 adapter 里重写 fetch。
- 组合式 API 与 `ref` 同步需你自己写；本包不提供响应式包装。

## 与生产应用的差异

生产有 SFC、Vite/Vue 插件与路由；adapter SSR / Nuxt 未接线。本示例无组件、无页面。

依赖：`@wae/client`、`@wae/adapter-vue`。
