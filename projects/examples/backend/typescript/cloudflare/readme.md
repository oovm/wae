# @wae-example/backend-ts-cloudflare

## 这个示例展示什么

Cloudflare Workers 面：依赖 `@wae/server-cloudflare`（`createCloudflareApp` / `createWorker` 一类 API）。源码尚未调用这些工厂。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。
- 无 `wrangler.toml`、无 `wrangler dev` 脚本；不要发明已接线的 Workers 流程。

## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/backend-ts-cloudflare run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts`
- `package.json` — 含 `@wae/server-cloudflare`

## 请求 / 事件路径（目标语义）

```text
createServer
  → createCloudflareApp / adapt
  → Worker fetch(request, env, ctx)
  → Response
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- `route` + `app.fetch` 验证 handler 与平台无关。
- 阅读 `@wae/server-cloudflare` README，对照 Worker 签名与 `env` 绑定。
- 与 `backend/typescript/fetch` 对比：那边更泛化 serverless，这边钉死 CF。

## 与生产应用的差异

生产用 Wrangler 发布与 KV/R2 绑定；本示例无 Worker 打包。

依赖（本示例）：`@wae/server`、`@wae/serverless`、`@wae/server-cloudflare`。
