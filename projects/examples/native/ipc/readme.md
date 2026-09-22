# @wae-example/native-ipc

## 这个示例展示什么

原生 IPC 边界：`createClient` 时设置 `env: { target: "desktop", hasNativeBridge: true }`，假定走 native bridge，而不是纯浏览器 HTTP 后端。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/native-ipc run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — `hasNativeBridge: true`
- `package.json`

## 请求 / 事件路径（目标语义）

```text
前端 ClientMessage
  → bridge / IPC
  → Rust host（wae-bridge）
  → native capability
  → HostMessage 回前端
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 对照 `minimal/bridge-only` 的路径图，确认「bridge → host」而不是 `createServer`。
- 在类型上查看 `client.native`；不要把未鉴权的 native 消息当成已授权系统调用。
- **不要**在本示例练习点里加 HTTP `route`——那会混进错误分层。

## 与生产应用的差异

生产需真实 postMessage transport、host 鉴权与壳进程；本示例未打开窗口。

依赖（本示例）：`@wae/client`。
