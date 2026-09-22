# `@wae/adapter-react`

把已有的 `@wae/client` 实例注入 **React** 树。本包只做 Provider / hook 边界，**不**提供 UI 组件，也**不**替代 `createClient`。

## 安装

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-react@0.0.0
pnpm add react react-dom   # peer，请自选已验证的主版本
```

## 提供什么

| 导出 | 角色 |
|------|------|
| `react()` | `defineConfig({ frontend: { adapter: react() } })` 工厂，返回 `{ name: "react" }` |
| `WaeProvider` | 预期接收 `client` + `children`，向下提供 context |
| `useWae()` | 在子树中取回 `WaeClient` |

## 与 client 的关系

```text
createClient(...)     ← 运行时（本包不创建）
  → <WaeProvider client={...}>
       useWae()       ← 读同一实例
```

## 最小结构（接线完成后）

以下展示调用关系；**0.0.0 中 `WaeProvider` 返回 `null`，`useWae` 会抛错**，不能直接复制运行。

```tsx
import { createClient } from "@wae/client";
import react, { WaeProvider, useWae } from "@wae/adapter-react";

const client = createClient({ server: { baseUrl: "/api" } });

function Greeter() {
  const wae = useWae();
  return <button onClick={() => wae.server.fetch("/hello")}>ping</button>;
}

export function App() {
  return (
    <WaeProvider client={client}>
      <Greeter />
    </WaeProvider>
  );
}

export default react;
```

## SSR / hydration / 状态

| 能力 | 0.0.0 |
|------|-------|
| SSR | 未实现 |
| hydration | 未实现 |
| 与 React state 同步 | 未实现；请自行用 `useState` / `useEffect` 包一层请求 |
| 错误与 loading | 未内置；在组件内处理 `server.action` 抛错 |

React 的并发与 Strict Mode 下重复 mount 行为，需在真实 Provider 实现里单独处理；当前骨架未覆盖。

## 相关

- [`@wae/client`](../../runtime/readme.md)
- 示例：[`examples/integration/react`](../../../examples/integration/react/readme.md)
