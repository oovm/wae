# `@wae/wae`

唯一项目 CLI 与 `defineConfig`。编排 Vite / Cargo / 平台包的入口，**不**内置 TSX 编译器，也**不**把 Rust runtime 塞进前端。

## 安装

```bash
pnpm add -D @wae/wae@0.0.0
```

安装后会按平台尝试拉取 `@wae/wae-*`（`optionalDependencies`）。二进制名：`wae`。

## CLI

```text
wae create <name>
wae dev [--platform <id>]
wae build [--platform <id>]
wae preview
wae run [--platform <id>]
wae check
wae test
wae generate [types]
wae help
```

**现状（0.0.0）**：除 help 外只打印骨架提示，未接线真实工具链。没有 `wae init`。

仓库内调试：

```bash
pnpm exec wae help
# 或
pnpm run wae -- help
```

## `defineConfig`

```ts
import { defineConfig } from "@wae/wae";

export default defineConfig({
  frontend: {
    framework: "none", // vue | react | svelte | solid | none
    // adapter: react(), // 来自 @wae/adapter-react 默认导出
    entry: "./src/main.ts",
  },
  server: {
    entry: "./server/index.ts",
    adapter: "cloudflare", // node | deno | cloudflare | bun
  },
  target: "web", // web | desktop | mobile
  platform: {
    client: "web",
    server: "cloudflare",
  },
});
```

规范化行为：`framework` 默认 `"none"`，`target` 默认 `"web"`。

| 字段 | 影响 |
|------|------|
| `frontend.*` | 前端入口与 adapter 选择 |
| `server.*` | 服务端入口与 runtime adapter |
| `target` | 是否走桌面/移动壳路径 |
| `platform.client` | 对应 `@wae/wae-*` 的 platform id |

模板见 `templates/app/`（`wae create` 接线后使用）。

## 开发 vs 生产（目标语义）

| 模式 | 目标行为 | 现状 |
|------|----------|------|
| `dev` | 热更新 + 平台调试壳 | 骨架 |
| `build` | 产出前端静态资源 + 平台产物 | 骨架 |
| `preview` / `run` | 本地预览 / 按平台运行 | 骨架 |

## 失败时查什么

1. 命令是否在 help 列表中。
2. `defineConfig` 字段类型是否合法。
3. 目标平台 optional 是否因 `os`/`cpu` 未安装。
4. 是否误期望 CLI 已生成完整工程（当前不会）。

## 相关

- 应用区说明：[`../readme.md`](../readme.md)
- 客户端：[`../../frontend/runtime/readme.md`](../../frontend/runtime/readme.md)
- 平台：[`../../platform/readme.md`](../../platform/readme.md)
