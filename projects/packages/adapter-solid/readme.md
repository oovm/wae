# `@wae/adapter-solid`

把 `@wae/client` 注入 **Solid** 树。Solid 使用细粒度响应式，**不要**假设 React 的重渲染模型；本包只规划 Provider / `useWae` 边界。

## 安装

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-solid@0.0.0
pnpm add solid-js   # peer
```

## 提供什么

| 导出 | 角色 |
|------|------|
| `solid()` | `{ name: "solid" }`，给 `defineConfig` |
| `WaeProvider` | 预期用 Solid `createContext` 提供 client |
| `useWae()` | 在子树中取 `WaeClient` |

与 React adapter **同名不同实现**：Solid 的 JSX 与 `createSignal` 生命周期不同，禁止混用 `@wae/adapter-react`。

## 最小结构（接线完成后）

以下展示调用关系；**0.0.0 中 `WaeProvider` 返回 `null`，`useWae` 抛错**。

```tsx
import { createClient } from "@wae/client";
import solid, { WaeProvider, useWae } from "@wae/adapter-solid";

const client = createClient({ server: { baseUrl: "/api" } });

function Ping() {
  const wae = useWae();
  return <button type="button" onClick={() => wae.server.fetch("/hello")}>ping</button>;
}

export function App() {
  return (
    <WaeProvider client={client}>
      <Ping />
    </WaeProvider>
  );
}

export default solid;
```

## SSR / hydration / 响应式

| 能力 | 0.0.0 |
|------|-------|
| SolidStart SSR | 未实现 |
| hydration | 未实现 |
| 与 `createResource` | 未封装；请自行包一层异步资源 |

## 相关

- [`@wae/client`](../client/readme.md)
- 示例：[`examples/integration/solid`](../../examples/integration/solid/readme.md)
