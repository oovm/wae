# `@wae/wae`

唯一项目 CLI 与 `defineConfig`。编排 Vite / Cargo / 平台包，**不**内置 TSX 编译器，也**不**把 Rust runtime 塞进前端。

## 安装

```bash
pnpm add -D @wae/wae@0.0.0
```

安装后会按平台尝试拉取 `@wae/wae-*`（`optionalDependencies`）。二进制名：`wae`。

## CLI

```text
wae create <name>
wae dev [--platform <id>] [--port <n>] [--host <addr>]
wae build [--platform <id>]
wae preview
wae run [--platform <id>] [--port <n>] [--host <addr>]
wae check
wae test
wae generate [types]
wae help
```

### 已接线：`run` / `dev`

在工程根（含 `wae.config.ts`）执行：

```bash
pnpm exec wae run
# 或
pnpm exec wae run --platform web --port 5173
```

行为：

1. 用 esbuild 加载 `wae.config.*`（经 `defineConfig` 规范化）。
2. `platform` / `target` 解析为客户端平台 id（默认 `web`）。
3. **`web`**：启动 Vite 开发服务器。若存在 `vite.config.*` 则沿用；否则按 `frontend.framework` 注入 `@vitejs/plugin-vue` / `@vitejs/plugin-react` 等。
4. **其它平台**：调用对应 `@wae/wae-*` 的 `platform.run()`（0.0.0 多为空实现）。

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

规范化：`framework` 默认 `"none"`，`target` 默认 `"web"`。

## 失败时查什么

1. 当前目录是否有 `wae.config.ts`。
2. `web` 时是否已安装 `vite`（以及对应框架的 Vite 插件）。
3. 是否缺少 `index.html`（Vite 根入口）。
4. 非 web 平台 optional 依赖是否因 `os`/`cpu` 未安装。

## 相关

- 应用区：[`../readme.md`](../readme.md)
- 可跑示例：[`vue-app`](../../examples/integration/vue-app/) · [`react-app`](../../examples/integration/react-app/)
- 平台：[`../../platform/readme.md`](../../platform/readme.md)
