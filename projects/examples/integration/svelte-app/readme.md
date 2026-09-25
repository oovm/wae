# @wae-example/integration-svelte-app

## 这个示例展示什么

**桌面 WebView + Svelte 5**：`target: "desktop"` → `wae run` 先起 Vite，再拉起 `wae-desktop`。

## 运行前提

- 仓库根已 `pnpm install`
- `pnpm --filter @wae/wae run build`
- `pnpm --filter @wae/adapter-svelte run build`
- `cargo build -p wae-desktop`（首次）
- Windows：已装 WebView2 Runtime

## 启动命令

```bash
pnpm --filter @wae-example/integration-svelte-app exec wae run --port 5175
```

仅浏览器：`wae run --platform web`。

## 关键文件

- `wae.config.ts` — `framework: "svelte"` + desktop
- `src/App.svelte` · `src/styles.css`
- 宿主：`projects/crates/wae-desktop`

依赖：`@wae/client` · `@wae/adapter-svelte` · `svelte` · Vite。
