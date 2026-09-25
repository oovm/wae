# protocol-only

## 这个示例展示什么

只关注 `@wae/protocol` / `@wae/types` / `@wae/core` 的消息形状与编解码，不构造 client 或 server。

## 运行前提

本目录**没有** `package.json` 与源码，不能对本文件夹 `pnpm run check`。需在仓库根已安装产品包，或临时 `pnpm add` 后自测。

## 启动命令

无本目录脚本。可在临时文件中：

```bash
pnpm add @wae/protocol@0.0.0 @wae/types@0.0.0 @wae/core@0.0.0
```

```ts
import { createRpcRequest, encodeMessage, decodeClientMessage } from "@wae/protocol";

const req = createRpcRequest("ping", { n: 1 });
const wire = encodeMessage({ type: "rpc", request: req });
const back = decodeClientMessage(wire);
void back;
```


## 访问地址

无浏览器地址。成功标准：上述编解码在你的脚本里跑通（或产品包自身测试）。

## 关键文件

- 目前仅本 README
- 实现见 [`@wae/protocol`](../../packages/protocol/readme.md)

## 请求 / 事件路径（目标语义）

```text
createRpcRequest / ClientMessage
  → encodeMessage
  → bytes / JSON wire
  → decode*Message
  → HostMessage / RpcResponse
```

## 练习点

- 对照 `@wae/types` 里 `RpcRequest`、`HostMessage` 字段。
- 与 [`host/bridge`](../../../crates/wae-bridge/readme.md) 的 `handle` 对照：协议是形状，bridge 是投递。
- 不要把编解码示例误当成 HTTP `createServer` 路由练习。

## 与生产应用的差异

生产编解码须与 schema / Rust 侧类型同步；本目录不是可发布示例包，也不走 pnpm filter。

依赖说明：无本目录依赖清单。练习时使用 `@wae/protocol`、`@wae/types`、`@wae/core`。
