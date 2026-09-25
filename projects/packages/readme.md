# packages

全部可发布的 `@wae/*` npm 包，扁平目录（目录名 ≠ 包名时以 `package.json` 的 `name` 为准）。

| 目录 | npm 包 | 角色 |
|------|--------|------|
| `wae` | `@wae/wae` | CLI · `defineConfig` |
| `client` | `@wae/client` | 框架无关前端 runtime |
| `adapter-vue` … `adapter-solid` | `@wae/adapter-*` | 框架注入 |
| `types` / `core` / `protocol` | `@wae/types` 等 | 通信层 |
| `server` / `serverless` | `@wae/server` 等 | 跨运行时服务端 |
| `server-node` … `server-cloudflare` | `@wae/server-*` | 运行时绑定 |
| `wae-web` / `wae-unknown-wasm32` | `@wae/wae-*` | 浏览器 / Wasm |
| `wae-win32-x64` 等 | `@wae/wae-*` | 桌面 / 移动平台壳 |

```bash
pnpm --filter @wae/wae run build
pnpm run check:boundary
```
