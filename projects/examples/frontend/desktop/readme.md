# @wae-example/frontend-desktop

## 这个示例展示什么

桌面 WebView 前端意图目录：应用跑在桌面壳里的页面侧。当前 `src/main.ts` 与 browser 一样只构造 `createClient`，尚未写入 `env.target`。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。
- 诚实状态：代码未区分桌面 env；目录名表达目标形态，不是已接线壳。

## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/frontend-desktop run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — `createClient` 骨架
- `package.json`

## 请求 / 事件路径（目标语义）

```text
桌面壳加载前端
  → createClient（页面内）
  →（可选）HTTP 到远程 server
  → 若启用 native：再经 bridge → host（见 native/*）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 对比 `frontend/browser`：同样只有 client 时，桌面多出来的是壳与可选 native，而不是另一套 `createClient`。
- 需要 `hasNativeBridge` 时去改或对照 `native/desktop-shell` / `native/ipc`。
- 不要指望 `wae dev` 弹出窗口——CLI 仍是骨架。

## 与生产应用的差异

生产依赖 `@wae/wae-win32-*` / `darwin-*` / `linux-*` 与 Rust host 二进制；本示例不启动壳。

依赖（本示例）：`@wae/client`。
