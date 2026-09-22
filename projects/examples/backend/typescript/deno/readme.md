# @wae-example/backend-ts-deno

## 这个示例展示什么

Deno 部署面：依赖 `@wae/server-deno`，目标是导出适配后的 `fetch`。源码仍只构造 `createServer()`。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。
- 本仓日常用 Node 22 跑 `tsc`；真正 `deno run` 需本机另装 Deno，且当前示例未提供 Deno 入口文件。

## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/backend-ts-deno run check
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
- `package.json` — 含 `@wae/server-deno`

## 请求 / 事件路径（目标语义）

```text
createServer
  → @wae/server-deno 适配
  → Deno.serve / 导出 fetch(request)
  → Response
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 用 `app.fetch` 在 Node 侧先测路由（类型检查仍走本包 `check`）。
- 阅读 `@wae/server-deno` 导出符号，写一个最小 `export default { fetch }` 草稿（可放本地临时文件）。
- 不要假设示例目录已含 `deno.json`。

## 与生产应用的差异

生产在 Deno Deploy / 自建 Deno 进程上跑；本示例未启动 Deno。

依赖（本示例）：`@wae/server`、`@wae/serverless`、`@wae/server-deno`。
