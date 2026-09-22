# @wae-example/native-mobile-shell

## 这个示例展示什么

移动壳意图目录。注意：当前 `src/main.ts` 仍写着 `target: "desktop"`（与其它 native 骨架相同），**尚未**改成 mobile——以源码为准。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。
- 诚实状态：env 字符串未反映 mobile；仅目录与包名表达意图。

## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/native-mobile-shell run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — 今日仍为 desktop + `hasNativeBridge`
- `package.json`

## 请求 / 事件路径（目标语义）

```text
移动壳
  → WebView
  → bridge → host
  → 移动系统能力（目标）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 把 `env.target` 改成文档中的 mobile 取值（若类型允许）并跑 `check`，观察类型约束。
- 对照 `frontend/mobile`（页面）与本目录（壳 / bridge）。
- 路径练习仍应围绕 bridge→host，而不是 `createServer`。

## 与生产应用的差异

生产需 Android/iOS 壳与商店分发；本示例无设备运行。

依赖（本示例）：`@wae/client`。
