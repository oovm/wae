# @wae-example/frontend-webview

## 这个示例展示什么

强调「跑在 WebView 里的前端」这一形态：与纯浏览器页共享 `@wae/client`，但后续常接到 host bridge。当前仍是构造-only 骨架。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/frontend-webview run check
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
WebView 文档
  → createClient
  → postMessage / bridge（目标）
  → host
  → HostMessage
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 把本目录当成「页面侧」，把 `minimal/bridge-only` / `native/ipc` 当成「桥与 host 侧」对照读。
- 在类型层面查看 `client.native` 是否在无 `hasNativeBridge` 时仍可用（以实际类型为准）。
- 勿把本示例当成已内嵌 WebView 控件的 demo。

## 与生产应用的差异

生产 WebView 由桌面/移动壳提供；本示例没有窗口、没有 transport。

依赖（本示例）：`@wae/client`。
