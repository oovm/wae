# @wae-example/native-desktop-shell

## 这个示例展示什么

桌面壳意图：页面在桌面 WebView 中，并启用 native bridge 标志。源码与 `native/ipc` 同形（`target: "desktop", hasNativeBridge: true`），目录强调「壳」而非单条 IPC 练习。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/native-desktop-shell run check
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
桌面壳进程
  → 加载 WebView 前端
  → bridge → host
  → 窗口 / 系统集成（目标）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 与 `frontend/desktop` 对照：那边可没有 native 标志；这边明确 `hasNativeBridge`。
- 阅读 `projects/crates` 与 `packages/wae-win32-*` 等平台包 README，弄清二进制从哪来。
- 不要运行虚构的 `wae open-desktop`。

## 与生产应用的差异

生产有打包安装包与自动更新；本示例无壳二进制启动。

依赖（本示例）：`@wae/client`。
