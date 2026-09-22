# @wae-example/integration-react-app

## 这个示例展示什么

可在浏览器打开的 **React + `wae run`** 工程：`defineConfig({ framework: "react" })`、`WaeProvider` / `useWae`、Vite。

## 运行前提

- 仓库根已 `pnpm install`
- `pnpm --filter @wae/wae run build`
- Node.js 22 + pnpm 10

## 启动命令

```bash
pnpm exec wae run --port 5174

# 或仓库根
pnpm --filter @wae-example/integration-react-app exec wae run --port 5174
```

cwd 须为本目录（含 `wae.config.ts`）。

## 访问地址

`http://127.0.0.1:5174/` — 应看到「WAE + React」与 ping 按钮。

## 关键文件

- `wae.config.ts` — `framework: "react"` + `adapter: react()`
- `index.html` / `src/main.tsx`
- `vite.config.ts` — `@vitejs/plugin-react`

## 请求 / 事件路径

```text
wae run
  → load wae.config.ts
  → Vite (web)
  → createClient + <WaeProvider>
  → useWae() 于 Panel
```

## 练习点

- 不要与 `@wae/adapter-vue` 的 provide 模型混用。
- 对比占位示例 [`../react`](../react/readme.md)。

## 与生产应用的差异

无 SSR、无真实 HTTP 后端；只验证 CLI 与 React 注入。

依赖：`@wae/client` · `@wae/adapter-react` · `react` · `@wae/wae` · `vite`。
