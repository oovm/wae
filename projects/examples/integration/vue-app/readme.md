# @wae-example/integration-vue-app

## 这个示例展示什么

可在浏览器打开的 **Vue + `wae run`** 工程：`defineConfig({ framework: "vue" })`、`provideWae` / `useWae`、Vite。

## 运行前提

- 仓库根已 `pnpm install`
- `pnpm --filter @wae/wae run build`（CLI 需要 `dist`）
- Node.js 22 + pnpm 10

## 启动命令

```bash
# 在本目录
pnpm exec wae run --port 5173

# 或在仓库根
pnpm --filter @wae-example/integration-vue-app exec wae run --port 5173
```

需在示例包目录执行，或保证 cwd 指向本目录（`wae` 在此找 `wae.config.ts`）。

## 访问地址

`http://127.0.0.1:5173/` — 应看到「WAE + Vue」与 ping 按钮。

## 关键文件

- `wae.config.ts` — `framework: "vue"` + `adapter: vue()`
- `index.html` / `src/main.ts` / `src/App.vue` — Vue 应用
- `vite.config.ts` — `@vitejs/plugin-vue`（`wae run` 会沿用）

## 请求 / 事件路径

```text
wae run
  → load wae.config.ts
  → Vite (web)
  → createClient + provideWae
  → useWae() 于组件
```

## 练习点

- 去掉本目录 `vite.config.ts`，确认 `wae run` 仍能按 framework 注入 Vue 插件。
- 对比仅边界占位的 [`../vue`](../vue/readme.md)。

## 与生产应用的差异

生产会接真实 server / 路由 / 构建发布；本页只验证 client 注入与 CLI。

依赖：`@wae/client` · `@wae/adapter-vue` · `vue` · `@wae/wae` · `vite`。
