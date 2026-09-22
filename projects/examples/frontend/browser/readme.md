# @wae-example/frontend-browser

## 这个示例展示什么

浏览器形态的前端占位：用 `@wae/client` 面向「普通网页」，默认走浏览器 bridge，不假定桌面/移动 native 壳。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/frontend-browser run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — 仅 `createClient({ server: { baseUrl: "/api" } })`
- `package.json`

## 请求 / 事件路径（目标语义）

```text
createClient（浏览器）
  → browser bridge / HTTP
  → 远程 createServer（本示例未包含）
  → Response → client 解码
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 保持 `env` 不设 `hasNativeBridge`，确认与 `native/*` 示例的差别在类型注释上。
- 自己起任意静态页 + 远程 API 时，把 `baseUrl` 换成真实 origin 再试 `server.fetch`。
- 需要 UI 框架时转 `integration/*`，不要在本目录加 adapter。

## 与生产应用的差异

生产会有 HTML/Vite 入口、真实页面与 CDN；本示例无页面、无端口。

依赖（本示例）：`@wae/client`。
