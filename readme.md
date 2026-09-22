# WAE

WAE 是一个由 **Rust host** 驱动、以 **TypeScript** 为应用层的 WebView 全栈框架。它提供框架无关的 `client` / `server`、通信协议与平台适配器；应用按目标环境组合浏览器、Wasm、桌面或移动壳。Rust 负责本地宿主与原生能力，不进入默认前端 bundle；Wasm 为显式 opt-in。

**没有** `@wae/ui`，**没有** `@wae/adapter-vanilla`。纯 TS / DOM 直接用 `@wae/client`。

当前 npm 版本为 `0.0.0`（占位首发）：库 API 与包面已对齐，CLI / 多数 adapter / 平台壳仍是骨架，不要按生产完备度理解。

## 产品组成（用户视角）

```text
应用配置（@wae/wae · defineConfig）
  ├─ frontend / client（@wae/client）
  │   └─ adapter：Vue / React / Svelte / Solid（可选）
  ├─ backend / server（@wae/server）
  │   └─ serverless → Node / Deno / Cloudflare
  ├─ communication（@wae/types · @wae/core · @wae/protocol）
  └─ host / platform
      ├─ @wae/wae-web
      ├─ @wae/wae-unknown-wasm32
      ├─ desktop：wae-win32-* / wae-darwin-* / wae-linux-*
      └─ mobile：wae-android-arm64 / wae-ios-arm64
```

| 概念 | 做什么 | 不是什么 |
|------|--------|----------|
| **client** | 前端运行时：`createClient`、HTTP/action、session、bridge | 不是 UI 框架 |
| **adapter** | 把已有 `WaeClient` 接到 Vue/React/Svelte/Solid | 不是第二套 runtime |
| **server** | 平台无关 `fetch` 应用与路由 | 不是 Node 进程本身 |
| **serverless** | 把 server 适配成 `(request, env, ctx) => Response` | 不是具体云厂商 API |
| **server-\*** | 绑定 Node / Deno / Cloudflare | 不可互相假设文件系统/定时器 |
| **protocol / types / core** | 跨端消息与共享原语 | 不是业务 handler |
| **host（Rust）** | 本地壳 ↔ 前端 bridge | 不是远程业务后端 |
| **platform（`@wae/wae-*`）** | 目标 OS/运行时发行包 | 不是普通业务依赖（经 CLI optional 拉取） |

## 选包

| 包 | 何时安装 |
|----|----------|
| `@wae/wae` | 需要 CLI 与 `defineConfig` |
| `@wae/client` | 任何前端（含无框架） |
| `@wae/adapter-vue` / `react` / `svelte` / `solid` | 只用对应框架时 |
| `@wae/server` | 写跨运行时 handler |
| `@wae/serverless` | 需要 Worker 风格 `fetch` 导出 |
| `@wae/server-node` / `server-deno` / `server-cloudflare` | 部署到对应运行时 |
| `@wae/types` / `@wae/core` / `@wae/protocol` | 协议/类型/ID 与编解码 |
| `@wae/wae-*` | 一般**不要**手装；装 `@wae/wae` 时作为 `optionalDependencies` 按平台拉取 |

不要安装：`@wae/ui`、`@wae/adapter-vanilla`（不存在且不会回来）。

## 按目标怎么选

- **只做浏览器前端**：`@wae/client` + 可选 adapter；平台用 `@wae/wae-web`（随 CLI）。
- **需要 Node HTTP 服务**：`@wae/server` + `@wae/server-node`（当前 `serve` 为骨架，未真正 `listen`）。
- **Deno**：`@wae/server` + `@wae/server-deno`（导出适配后的 `fetch`）。
- **Cloudflare Worker**：`@wae/server` + `@wae/server-cloudflare`（`createCloudflareApp` / `createWorker`）。
- **Wasm 客户端**：`@wae/wae-unknown-wasm32`；构建与加载方式见该包 README（当前为占位 API）。
- **桌面 WebView**：`target: "desktop"` + 对应 `@wae/wae-win32-*` / `darwin-*` / `linux-*` + Rust host；原生二进制尚未打进 npm。
- **移动壳**：`@wae/wae-android-arm64` / `@wae/wae-ios-arm64` + host；同样为占位。

## 十分钟能验证什么

### A. `wae run`（Vue / React 页）

```bash
pnpm install
pnpm --filter @wae/wae run build
pnpm --filter @wae-example/integration-vue-app exec wae run --port 5173
# 另开终端：
pnpm --filter @wae-example/integration-react-app exec wae run --port 5174
```

浏览器打开终端打印的 Local URL，应看到标题与「ping client」按钮。

### B. 库 API（无需进程）

```ts
import { createServer, route } from "@wae/server";
const app = createServer({
  routes: [route("GET", "/hello", (ctx) => ctx.json({ ok: true }))],
});
await app.fetch(new Request("http://x/hello")); // → JSON
```

```bash
pnpm run check:boundary
pnpm run check:ts
pnpm exec wae help
```

`wae create` / `build` / `generate` 等仍是骨架。
## 请求如何流动

**浏览器 ↔ 远程 server（HTTP）**

```text
createClient → server.fetch / action
  → HTTP
  → createServer 路由 handler
  → Response
  → client 解码（action 期望 JSON）
```

**前端 ↔ Rust host（WebView / native）**

```text
ClientMessage（@wae/protocol）
  → bridge / IPC
  → host（wae-bridge）
  → HostMessage（domPatch / rpc / error）
```

**Worker 部署**

```text
createServer → adaptFetch / createCloudflareApp
  → runtime 的 fetch(request, env, ctx)
```

## 下一步读哪份 README

| 你要做的事 | 去读 |
|------------|------|
| 配置工程 / CLI | [`projects/application/wae`](projects/application/wae/readme.md) |
| 前端 runtime | [`@wae/client`](projects/frontend/runtime/readme.md) |
| 框架接入 | [`adapters`](projects/frontend/adapters/readme.md) |
| 服务端 | [`@wae/server`](projects/backend/server/readme.md) |
| 通信与协议 | [`communication`](projects/communication/readme.md) |
| Rust host | [`host`](projects/host/readme.md) |
| 平台包 | [`platform`](projects/platform/readme.md) |
| 可对照示例 | [`examples`](projects/examples/readme.md) |

## 仓库检查与发布

```bash
pnpm run check:boundary
pnpm run fmt:check
pnpm run publish:dry
```

`0.0.0` 发布范围：产品面、通信、后端适配、框架 adapter、全部 `@wae/wae-*`。不发布 examples。
