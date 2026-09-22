# `@wae/wae`

唯一项目 CLI 与 `defineConfig`。编排前端工具链 / Cargo / 平台包，**不**内置 TSX 编译器，也**不**把 Rust runtime 塞进前端。

## 与 Vite 的关系

与 Tauri 常见模板类似：**常用 Vite，但不绑死 Vite**。

| 配置 | 行为 |
|------|------|
| `frontend.bundler: "vite"`（**默认**） | `wae run` / `wae dev` 代启 Vite；`vite` 为 optional peer |
| `frontend.bundler: "custom"` | 不代启任何 bundler；你用 Webpack / Rspack / Parcel 等，并用 `frontend.devUrl` 标明开发地址 |

换工具链时改的是工程脚本与 `bundler` / `devUrl`，不是换掉 `@wae/client` 或 adapter。

## 安装

```bash
pnpm add -D @wae/wae@0.0.0
# 常用路径再装 Vite
pnpm add -D vite@^7
```

安装后会按平台尝试拉取 `@wae/wae-*`（`optionalDependencies`）。二进制名：`wae`。

## CLI

```text
wae create <name>
wae dev [--platform <id>] [--port <n>] [--host <addr>] [--open|--no-open]
wae build [--platform <id>]
wae preview
wae run [--platform <id>] [--port <n>] [--host <addr>] [--open|--no-open]
wae check
wae test
wae generate [types]
wae help
```

### 已接线：`run` / `dev`

```bash
pnpm exec wae run
pnpm exec wae run --platform web --port 5173
```

行为：

1. 用 esbuild 加载 `wae.config.*`（经 `defineConfig` 规范化）。
2. 解析 `platform` / `target` 为客户端平台 id（默认 `web`）。
3. **`web` + `bundler: "vite"`**：启动 Vite。有 `vite.config.*` 则沿用；否则按 `frontend.framework` 注入官方插件。
4. **`web` + `bundler: "custom"`**：不启 Vite，打印 `devUrl` 提示。
5. **其它平台**：调用对应 `@wae/wae-*` 的 `platform.run()`（0.0.0 多为空实现）。

`dev` 与 `run` 当前同一实现。`create` / `build` / `preview` / `check` / `test` / `generate` 尚未接线。

仓库内调试：

```bash
pnpm --filter @wae/wae run build
pnpm --filter @wae-example/integration-vue-app exec wae run --port 5173
```

## `defineConfig`

```ts
import { defineConfig } from "@wae/wae";

export default defineConfig({
  frontend: {
    framework: "vue", // vue | react | svelte | solid | none
    // adapter: vue(),
    entry: "./src/main.ts",
    bundler: "vite", // 默认；换工具链时用 "custom" + devUrl
    // bundler: "custom",
    // devUrl: "http://127.0.0.1:8080",
  },
  server: {
    entry: "./server/index.ts",
    adapter: "node",
  },
  target: "web",
  platform: {
    client: "web",
    server: "node",
  },
});
```

规范化：`framework` 默认 `"none"`，`bundler` 默认 `"vite"`，`target` 默认 `"web"`。

## 失败时查什么

1. 当前目录是否有 `wae.config.ts`。
2. `bundler: "vite"` 时是否已安装 `vite`（以及对应框架插件）。
3. 是否缺少 `index.html`（Vite 根入口）。
4. `bundler: "custom"` 时是否已自启工具链，且 `devUrl` 可访问。
5. 非 web 平台 optional 依赖是否因 `os`/`cpu` 未安装。

## 相关

- 应用区：[`../readme.md`](../readme.md)
- 可跑示例：[`vue-app`](../../examples/integration/vue-app/) · [`react-app`](../../examples/integration/react-app/)
- 平台：[`../../platform/readme.md`](../../platform/readme.md)
