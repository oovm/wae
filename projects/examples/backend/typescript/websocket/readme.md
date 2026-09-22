# @wae-example/backend-ts-websocket

## 这个示例展示什么

后端 WebSocket 意图（无 WAE 前端）：依赖面同泛化 server/serverless。源码未建立 WS 服务器。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/backend-ts-websocket run check
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
WS 客户端
  → 后端升级或独立 WS listener（目标）
  → 帧处理
  → 回推
（今日仅有 createServer 占位）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 先完成 HTTP `route` 自测，再查所选运行时（Node/Deno）的 WS API 如何与 `fetch` 应用并存。
- 对照 `fullstack/websocket`：本目录没有 `@wae/client`。
- 避免把空的 `createServer()` 说成「已支持 WS」。

## 与生产应用的差异

生产要连接管理与水平扩展；本示例无监听。

依赖（本示例）：`@wae/server`、`@wae/serverless`。
