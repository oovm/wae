# @wae-example/minimal-client-only

## 这个示例展示什么

最小前端边界：只 `import { createClient } from "@wae/client"` 并构造实例。不引入 `@wae/server`、adapter、也不打开 native bridge。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/minimal-client-only run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — `createClient({ server: { baseUrl: "/api" } })`
- `package.json` — 仅依赖 `@wae/client`

## 请求 / 事件路径（目标语义）

```text
createClient({ server: { baseUrl } })
  → WaeClient（本示例未发任何 HTTP / action）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 改 `baseUrl`，观察类型是否仍通过 `check`。
- 阅读 `@wae/client` 的 `server.fetch` / `server.action` 签名，在**自备** HTTP 服务上再试真实请求。
- **不要**在本示例里加 `createServer`：那是 `minimal/server-only` 与 fullstack 的事。

## 与生产应用的差异

生产还会配真实 origin、错误处理、可选 `@wae/adapter-*`，以及静态托管或壳。本目录故意保持「只有 client」。

依赖（本示例）：`@wae/client`。
