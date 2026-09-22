# @wae-example/backend-ts-fetch

## 这个示例展示什么

纯 `fetch` 适配意图：`@wae/server` + `@wae/serverless`，不绑定 Node/Deno/CF 专用包。适合任何能调用 `app.fetch` 的运行时。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/backend-ts-fetch run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — `createServer()`
- `package.json` — 无 `server-node` / `server-deno` / `server-cloudflare`

## 请求 / 事件路径（目标语义）

```text
外部运行时拿到 Request
  → adaptFetch / app.fetch
  → handler
  → Response
（无 listen，无特定云厂商 API）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 声明路由后直接 `await app.fetch(request)`，把结果当单元测试。
- 刻意不要添加 `@wae/server-node`，保持「平台无关」依赖面。
- 需要进程或 Worker 时再分支到 node/deno/cloudflare 示例。

## 与生产应用的差异

生产仍要选一个具体宿主；本示例停在可移植 `fetch` 边界。

依赖（本示例）：`@wae/server`、`@wae/serverless`。
