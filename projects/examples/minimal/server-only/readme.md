# @wae-example/minimal-server-only

## 这个示例展示什么

最小服务端边界：只 `createServer()`，不挂 `@wae/client`、不绑定 Node/Deno/Cloudflare 适配器。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/minimal-server-only run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — `createServer()` 后 `void app`
- `package.json` — 仅依赖 `@wae/server`

## 请求 / 事件路径（目标语义）

```text
createServer({ routes? })
  → WaeServerApp.fetch(Request)
  → Response
（本示例未调用 fetch，也未 listen）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 用 `route("GET", "/hello", …)` 声明路由，再 `app.fetch(new Request("http://x/hello"))` 断言 JSON。
- 对比根 README「十分钟能验证什么」里的 server 片段。
- 需要进程监听时再看 `backend/typescript/node`，不要在本目录假造 `wae dev`。

## 与生产应用的差异

生产会声明完整 routes / middleware，并用 `@wae/server-node` 等接到运行时。本示例停在平台无关 `createServer`。

依赖（本示例）：`@wae/server`。
