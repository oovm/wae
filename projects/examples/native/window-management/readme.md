# @wae-example/native-window-management

## 这个示例展示什么

窗口管理意图：最小化、多窗、尺寸等由 host/壳处理。当前仅启用 native bridge 标志。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/native-window-management run check
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
前端 UI 动作
  → window.* 类 ClientMessage（目标）
  → bridge → host
  → 壳调整 OS 窗口
  → 可选 HostMessage 回执
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 与 `native/desktop-shell` 一起读：壳负责窗口生命周期，本目录聚焦窗口控制消息。
- `check` 通过即今日验收；不要截图「假窗口」。
- 练习路径保持 bridge→host。

## 与生产应用的差异

生产有多显示器、全屏与平台差异；本示例无窗口。

依赖（本示例）：`@wae/client`。
