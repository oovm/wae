# @wae-example/backend-ts-node

## 这个示例展示什么

无 WAE 前端的 Node 后端面：依赖 `@wae/server` + `@wae/serverless` + `@wae/server-node`。`src/main.ts` 目前只 `createServer()`，未调用 `serve`。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/backend-ts-node run check
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
- `package.json` — 含 `@wae/server-node`

## 请求 / 事件路径（目标语义）

```text
createServer
  →（目标）@wae/server-node serve(app)
  → Node HTTP 进程
  → app.fetch 处理 Request
（0.0.0 的 serve 为骨架，可能不 listen）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 先 `route` + `app.fetch` 不经过 Node 进程验证 handler。
- 阅读 `@wae/server-node` README，看 `serve` 当前是否仍打印骨架。
- 本示例**没有** `createClient`——不要照抄 fullstack 练习。

## 与生产应用的差异

接线后预期长驻 Node 进程与真实端口；当前无端口。

依赖（本示例）：`@wae/server`、`@wae/serverless`、`@wae/server-node`。
