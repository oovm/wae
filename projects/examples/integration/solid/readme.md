# @wae-example/integration-solid

## 这个示例展示什么

Solid 注入边界：依赖 `@wae/client` + `@wae/adapter-solid`。API 名虽有 `WaeProvider` / `useWae`，但建立在 Solid 的 **细粒度响应式** 上，**不要**用 React 重渲染心智套用，也**禁止**混用 `@wae/adapter-react`。本目录无 Solid JSX 挂载。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10
- peer：`solid-js`（自行安装）
- 0.0.0：Provider / hook 为骨架（可能 `null` / 抛错）

## 启动命令

```bash
pnpm --filter @wae-example/integration-solid run check
```

## 访问地址

**无。** 成功标准：`tsc --noEmit` 通过。

## 关键文件

- `src/main.ts` — `createClient` + `import * as adapter from "@wae/adapter-solid"`
- 包说明：[`@wae/adapter-solid`](../../../frontend/adapters/solid/readme.md)

## 请求 / 事件路径

```text
createClient
  → <WaeProvider client={client}>   ← Solid context（非 React）
  → useWae()
  → client.server.fetch / action → HTTP → 远程 server
```

## 练习点

- 打开 `@wae/adapter-solid` README；异步数据用 Solid `createResource` 自包一层（adapter 未封装）。
- 不要从 React 示例复制 JSX 到 Solid 工程并期望同一 runtime。
- SolidStart SSR / hydration 未实现。

## 与生产应用的差异

生产有 Solid 构建链；本示例无组件、无页面。

依赖：`@wae/client`、`@wae/adapter-solid`。
