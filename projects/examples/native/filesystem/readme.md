# @wae-example/native-filesystem

## 这个示例展示什么

经 host 访问文件系统的意图：前端不直接 `fs`，而是经 bridge 请求 native 能力。源码尚未发出任何 FS 消息。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/native-filesystem run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — desktop + `hasNativeBridge`
- `package.json`

## 请求 / 事件路径（目标语义）

```text
前端请求读/写路径
  → ClientMessage
  → bridge → host
  → 宿主 FS API
  → HostMessage（内容或错误）
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 查阅 protocol / host 文档里与文件相关的消息名（以仓库现状为准）。
- 用类型练习构造 client；真正 FS 需 host 实现。
- 禁止在练习点里改成 Node `fs` 冒充 WAE native——分层会错。

## 与生产应用的差异

生产有权限提示、沙箱路径与用户授权；本示例无磁盘 IO。

依赖（本示例）：`@wae/client`。
