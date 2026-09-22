# @wae-example/integration-vue-app

## 这个示例展示什么

**桌面 WebView + Vue**：`target: "desktop"` → `wae run` 先起 Vite，再拉起 `wae-desktop` 原生窗口加载该页。

## 运行前提

- 仓库根已 `pnpm install`
- `pnpm --filter @wae/wae run build`
- `cargo build -p wae-desktop`（首次）
- Windows：已装 WebView2 Runtime
- Node.js 22 + pnpm 10

## 启动命令

```bash
# 仓库根或本目录 cwd 指向本包
pnpm --filter @wae-example/integration-vue-app exec wae run --port 5173
```

默认读 `wae.config.ts` 的 `target: "desktop"` / `platform.client: "win32-x64"`。  
仅浏览器试跑：`wae run --platform web`。

## 访问地址

原生窗口标题「WAE · vue」，内容与 Vite 页相同（「WAE + Vue」/ ping）。**不是**系统默认浏览器标签页。

## 关键文件

- `wae.config.ts` — `target: "desktop"`
- `src/App.vue` · `src/main.ts`
- 宿主：`projects/host/desktop`（`wae-desktop`）

## 请求 / 事件路径

```text
wae run
  → Vite（frontend）
  → @wae/wae-win32-x64.run({ url })
  → cargo/二进制 wae-desktop --url …
  → WebView2 窗口
```

## 练习点

- 关窗口应顺带结束 Vite。
- 改 `target: "web"` 对比浏览器路径。

## 与生产应用的差异

生产会捆预构建静态资源进壳；本示例开发期旁路 Vite URL。

依赖：`@wae/client` · `@wae/adapter-vue` · `vue` · `@wae/wae` · `vite` · 本机 `wae-desktop`。
