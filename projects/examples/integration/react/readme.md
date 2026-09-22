# @wae-example/integration-react

## 这个示例展示什么

React 注入边界：依赖 `@wae/client` + `@wae/adapter-react`。正式注入是 JSX **`WaeProvider`** + hook **`useWae`**。本目录只构造 client 并 import adapter，**没有** React 组件树、也没有 `react-dom` 挂载。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10
- peer：`react` / `react-dom`（包声明 React 19 量级；自行安装）
- 0.0.0：`WaeProvider` 可返回 `null`，`useWae` 抛错——属骨架预期

## 启动命令

```bash
pnpm --filter @wae-example/integration-react run check
```

## 访问地址

**无。** 成功标准：`tsc --noEmit` 通过。

## 关键文件

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-react"`
- 包说明：[`@wae/adapter-react`](../../../frontend/adapters/react/readme.md)

## 请求 / 事件路径

```text
createClient
  → <WaeProvider client={client}>   ← React 特有入口
  → useWae() 于子组件
  → client.server.fetch / action → HTTP → 远程 server
```

## 练习点

- 打开 `@wae/adapter-react` README：导出是 `WaeProvider` / `useWae` / `react()`。
- **禁止**与 `@wae/adapter-solid` 混用同名 API（实现不同）。
- Strict Mode 双调用、并发下的 Provider 行为要在真实实现里单独处理；骨架未覆盖。
- loading / 错误：用组件内 `useState` 包 `server.action`，adapter 不内置。

## 与生产应用的差异

生产有完整 React 工程；SSR / hydration 未实现。本示例无 JSX 运行。

依赖：`@wae/client`、`@wae/adapter-react`。
