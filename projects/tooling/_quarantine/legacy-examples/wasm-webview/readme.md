# wasm-webview

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/wasm-webview dev
```

## 构建

```bash
pnpm --filter @wae-example/wasm-webview build
```

## 覆盖

- @wae/wae-unknown-wasm32
- WASM 加载契约
- 浏览器/WebView bridge
- fetch / WebSocket
- 静态部署

## 不覆盖

- 原生窗口
- 文件系统
