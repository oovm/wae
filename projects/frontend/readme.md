# frontend

给**前端开发者**：用 `@wae/client` 的 `createClient` 拿到框架无关运行时；需要 Vue / React / Svelte / Solid 时再装对应 `@wae/adapter-*`。

## 先分清两层

| 层 | 包 | 负责 | 不负责 |
|----|-----|------|--------|
| **runtime** | `@wae/client` | HTTP/action、session、lifecycle、navigation、native bridge、WebSocket 工厂 | JSX、组件、hooks、signals、CSS |
| **adapter** | `@wae/adapter-*` | 把已有 `WaeClient` 注入框架上下文 | 替代 `createClient` |

无框架：只装 `@wae/client`。有框架：`createClient` 一次，再交给 adapter。

## 目录

| 路径 | 说明 |
|------|------|
| [`runtime/`](runtime/readme.md) | `@wae/client` |
| [`adapters/`](adapters/readme.md) | 四个框架 adapter 索引 |
| [`devtools/`](devtools/readme.md) | 开发期辅助（非产品运行时核心） |

## 浏览器 vs WebView

- **浏览器**：默认 `createBrowserBridge`；无原生文件/窗口能力。
- **WebView / native**：环境检测到 native bridge 时走 `createNativeIpcBridge`；能力经 host 鉴权，前端 `native` 消息不可默认可信。

细节见 [`runtime/readme.md`](runtime/readme.md)。
