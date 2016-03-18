# WAE

**Rust 驱动的 WebView 全栈框架。**

```text
@wae/wae      CLI（defineConfig；编排，不替代框架 CLI）
@wae/client   框架无关 TypeScript 运行时
@wae/server   平台无关服务端模型
```

扩展：`@wae/adapter-{vue,react,svelte,solid}` · `serverless` · `server-*` · 平台壳。  
**无** `@wae/ui`、**无** `@wae/adapter-vanilla`。

## 用法

```ts
import { createClient } from "@wae/client";
const client = createClient({ server: { baseUrl: "/api" } });
```

```ts
import { defineConfig } from "@wae/wae";
export default defineConfig({
  frontend: { framework: "react" },
  server: { entry: "./server/index.ts" },
  target: "desktop",
});
```

## 目录（功能区）

```text
projects/{application,frontend,backend,host,platform,communication,tooling,examples}/
```

examples 按用途：`frontend` / `fullstack` / `backend` / `native` / `integration` / `minimal`。

## 检查

```bash
pnpm run check:boundary
```
