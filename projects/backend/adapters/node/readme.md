# `@wae/server-node`

把 `@wae/server` 接到 **Node.js / Bun 长驻进程**。文件系统、TCP listen、进程级定时器等只应出现在本包（或你自己的 Node 代码），不要写进 `@wae/server`。

## 安装

```bash
pnpm add @wae/server@0.0.0 @wae/server-node@0.0.0
```

## 入口

```ts
import { createServer, route } from "@wae/server";
import { serve } from "@wae/server-node";

const app = createServer({
  routes: [route("GET", "/health", (ctx) => ctx.json({ ok: true }))],
});

const handle = serve(app, { port: 3000, hostname: "127.0.0.1" });
console.log(handle.port, handle.hostname);
await handle.close();
```

| API | 说明 |
|-----|------|
| `serve(app, options?)` | 返回 `ServeHandle`：`port`、`hostname`、`close()` |
| `options.port` | 默认 `3000` |
| `options.hostname` | 默认 `127.0.0.1` |

## 当前限制（0.0.0）

**不会**打开 TCP 端口，也**不会**调用 `node:http` / `Bun.serve`。`serve` 只登记配置并返回可 `close` 的句柄。验证路由请直接：

```ts
await app.fetch(new Request("http://127.0.0.1:3000/health"));
```

## 与其它运行时的差异

| 能力 | Node（本包目标） | Deno / Cloudflare |
|------|------------------|-------------------|
| 长驻 listen | 是（接线后） | 通常导出 fetch |
| 文件系统 | 可用 Node API | 受限或不同 API |
| 部署 | 自管进程 / 容器 | `deno` / `wrangler` |

## 环境

- 目标：Node.js 22+（与仓库验证环境一致）；Bun 计划兼容，未在 CI 单测。
- 部署命令：接线完成后预期为 `node dist/server.js` 一类；**当前无官方启动脚本**。

## 相关

- 抽象层：[`@wae/server`](../../server/readme.md)
- 示例：[`examples/backend/typescript/node`](../../../examples/backend/typescript/node/readme.md)
