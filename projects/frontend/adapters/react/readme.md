# `@wae/adapter-react`

把已有的 `@wae/client` 注入 **React** 树。本包只做 Provider / hook，**不**提供 UI 组件。

## 安装

```bash
pnpm add @wae/client@0.0.0 @wae/adapter-react@0.0.0
pnpm add react react-dom   # peer ^19
```

## 提供什么

| 导出 | 角色 |
|------|------|
| `react()` | `{ name: "react" }`，给 `defineConfig` |
| `WaeProvider` | `client` + `children` 的 Context Provider |
| `useWae()` | 子树取回 `WaeClient`；缺 Provider 时抛错 |

## 最小用法

```tsx
import { createClient } from "@wae/client";
import react, { WaeProvider, useWae } from "@wae/adapter-react";

const client = createClient({ server: { baseUrl: "/api" } });

function Greeter() {
  const wae = useWae();
  return <button type="button" onClick={() => void wae.server.fetch("/hello")}>ping</button>;
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

| 能力 | 现状 |
|------|------|
| SSR | 未实现 |
| hydration | 未实现 |
| 错误 / loading | 组件内自行处理 |

## 相关

- [`@wae/client`](../../runtime/readme.md)
- 可跑示例：[`examples/integration/react-app`](../../../examples/integration/react-app/readme.md)
