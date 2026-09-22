# @wae-example/frontend-mobile

## 这个示例展示什么

移动壳内前端意图：页面跑在 Android/iOS WebView。当前源码同样只是 `createClient` 骨架，未设置 mobile env。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。
- 诚实状态：无移动模拟器流程；无 `@wae/wae-android-*` / `ios-*` 启动脚本。

## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/frontend-mobile run check
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
移动壳 WebView
  → createClient
  → HTTP 与/或 native bridge → host
  → HostMessage 回页面
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 阅读平台包 README，弄清 mobile 包是 optional 发行物，不是本示例依赖。
- 对照 `native/mobile-shell`（IPC/壳侧意图）与本目录（页面侧意图）。
- 练习改 `baseUrl` 类型检查即可；不要编造 `adb`/`xcode` 启动命令。

## 与生产应用的差异

生产需真实移动壳与签名分发；0.0.0 平台包多为占位。

依赖（本示例）：`@wae/client`。
