# bridge-only

## 这个示例展示什么

只关注前端 bridge 与 Rust host 之间的边界：消息进 host，而不是进远程 `@wae/server`。

## 运行前提

本目录**没有** `package.json` / `src`。可测的是 crate `wae-bridge`，以及其它示例里 `env.hasNativeBridge` 的类型面。

## 启动命令

```bash
cargo test -p wae-bridge
cargo check -p wae-bridge
```

前端侧可在临时文件或 [`native/ipc`](../native/ipc/readme.md) 对照：

```ts
import { createClient } from "@wae/client";

const client = createClient({
  server: { baseUrl: "/api" },
  env: { target: "desktop", hasNativeBridge: true },
});
void client.native.bridge;
```


## 访问地址

无。成功标准：`cargo test -p wae-bridge`；前端侧今天只能验证类型与默认 bridge 选择。

## 关键文件

- 本 README
- [`host/bridge`](../../../crates/wae-bridge/readme.md)
- 有代码的相近示例：[`native/ipc`](../native/ipc/readme.md)

## 请求 / 事件路径（目标语义）

```text
ClientMessage
  → MessageTransport / IPC（bridge）
  → host（wae-bridge）handle
  → Option<HostMessage>
  → 前端解码
```

## 练习点

- 分清「远程 `@wae/server`」与「本地 host」：二者都不是对方的别名。
- 跟读 `native/ipc` 的 `hasNativeBridge: true`，再回来对照本 README 的路径图。
- 0.0.0 的 `handle` 可能恒返回 `None`——练习目标是边界，不是功能清单。

## 与生产应用的差异

生产需要真实 WebView `postMessage` transport 与平台壳二进制；本目录本身无可运行前端。

依赖说明：无本目录 npm 包。Rust 侧看 `wae-bridge`；TS 侧练习依赖 `@wae/client`。
