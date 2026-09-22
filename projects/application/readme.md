# application

给**应用作者**：用 `@wae/wae` 的 `defineConfig` 声明前端、后端与运行目标，再用 CLI 编排工具链。

本目录不是业务后端，也不是 UI 框架。它回答：配置写在哪、CLI 有哪些命令、失败时先查什么。

## 包含什么

| 路径 | npm | 作用 |
|------|-----|------|
| [`wae/`](wae/readme.md) | `@wae/wae` | 唯一 CLI（`wae`）与 `defineConfig` |

## 配置如何影响各层

```ts
import { defineConfig } from "@wae/wae";

export default defineConfig({
  frontend: {
    framework: "react", // vue | react | svelte | solid | none
    entry: "./src/main.ts",
  },
  server: {
    entry: "./server/index.ts",
    adapter: "node", // node | deno | cloudflare | bun
  },
  target: "web", // web | desktop | mobile
  platform: {
    client: "web",
    server: "node",
  },
});
```

- `frontend.framework`：选 adapter；`none` = 只用 `@wae/client`。
- `server.adapter`：选 `@wae/server-*`，不改变 `@wae/server` 的 handler 写法。
- `target`：浏览器 / 桌面壳 / 移动壳；决定是否依赖对应 `@wae/wae-*` 与 Rust host。
- `platform.client`：具体客户端平台 id（与 `@wae/wae-*` 的 `wae.platform` 对齐）。

## CLI 现状

安装 `@wae/wae` 后二进制为 `wae`。命令面：`create`、`dev`、`build`、`preview`、`run`、`check`、`test`、`generate`。

**0.0.0**：除 `help` 外均为骨架（打印提示，不接 Vite / 平台包）。没有 `wae init`；脚手架命令名是 `create`。

出错时：先确认配置文件是否被 `defineConfig` 规范化、包是否装齐、目标平台 optional 依赖是否因 `os`/`cpu` 被跳过。

详见 [`wae/readme.md`](wae/readme.md)。
