# @wae-example/fullstack-ssr

## 这个示例展示什么

SSR 意图：server 渲染 HTML，再 hydrate 到 client。当前只有双构造骨架，**没有**模板引擎或 framework SSR 适配。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。
- adapter 文档写明框架 SSR 多未支持；本示例不证明 SSR 已可用。

## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/fullstack-ssr run check
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
- `package.json`

## 请求 / 事件路径（目标语义）

```text
Request
  → createServer 渲染 HTML（目标）
  → Response text/html
  → 浏览器 hydrate → createClient 接管交互
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 用 `route` 返回 `new Response("<html>…</html>", { headers: { "content-type": "text/html" } })`，经 `app.fetch` 查看正文。
- 分清「server 吐 HTML」与「integration adapter 的客户端注入」是两层。
- 不要调用不存在的 `wae ssr` CLI。

## 与生产应用的差异

生产需框架 SSR 管道、缓存与流式渲染；本示例无 HTML 出口。

依赖（本示例）：`@wae/client`、`@wae/server`、`@wae/serverless`。
