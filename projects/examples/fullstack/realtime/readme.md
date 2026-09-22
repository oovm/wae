# @wae-example/fullstack-realtime

## 这个示例展示什么

实时同步意图（订阅 / 推送 / 多端一致），比「单条 WebSocket 回显」更偏应用层。当前同样只是 client+server 构造。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/fullstack-realtime run check
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
client 订阅兴趣集
  → server 受理（HTTP 或 WS）
  → 事件总线 / 存储变更
  → 推送 → 多 client 更新
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 用内存 Map 模拟「房间」，先以多次 `app.fetch` 扮演推送前的命令通道。
- 与 `fullstack/websocket` 分工：本目录练「同步模型」，那边练「传输」。
- 不要引入第三方 realtime SaaS 当作 WAE 内置能力。

## 与生产应用的差异

生产有持久化、扇出与背压；本示例无事件总线实现。

依赖（本示例）：`@wae/client`、`@wae/server`、`@wae/serverless`。
