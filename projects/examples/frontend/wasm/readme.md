# @wae-example/frontend-wasm

## 这个示例展示什么

Wasm 客户端意图：前端可显式 opt-in Wasm 相关平台包。本示例依赖仍只有 `@wae/client`，**没有**加载 `.wasm` 的代码。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。
- Wasm 为显式 opt-in；不要假定默认 bundle 含 Rust/Wasm。

## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/frontend-wasm run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — 仅 `createClient`
- `package.json`

## 请求 / 事件路径（目标语义）

```text
页面 / Worker
  →（可选）加载 @wae/wae-unknown-wasm32 能力
  → createClient 仍走 TS API
  → 远程 HTTP 或其它 transport
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 阅读 `@wae/wae-unknown-wasm32` 包 README，分清「平台占位 API」与本示例的 client 构造。
- 确认本 `package.json` **未**依赖 wasm 平台包——这是刻意的最小面。
- 练习保持在 TS `check`；不要假装已有 wasm-pack 流水线。

## 与生产应用的差异

生产需构建与加载 Wasm 产物；当前平台 API 多为占位，本示例更不会跑 Wasm。

依赖（本示例）：`@wae/client`。
