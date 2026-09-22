# @wae-example/fullstack-rpc

## 这个示例展示什么

全栈 RPC / action 意图：同一入口文件里同时构造 `createClient` 与 `createServer`，演示「前后端契约同仓占位」。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/fullstack-rpc run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — 同时 `createClient` + `createServer`
- `package.json` — client / server / serverless

## 请求 / 事件路径（目标语义）

```text
client.server.action / fetch
  → HTTP（或未来同构调用）
  → createServer 路由 / handler
  → Response
  → client 解码（action 期望 JSON）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 给 server 加 `route`，用 `app.fetch` 自测；再设想 client 的 `server.action` 对齐同一路径。
- 保持 client 与 server 在同一文件只是骨架便利；生产应拆包。
- 对比 `minimal/*`：这里才是「两边都出现」的组合。

## 与生产应用的差异

生产会拆前后端工程、真实端口与鉴权；本示例未发请求、未 listen。

依赖（本示例）：`@wae/client`、`@wae/server`、`@wae/serverless`。
