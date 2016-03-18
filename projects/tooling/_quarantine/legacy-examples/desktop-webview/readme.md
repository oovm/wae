# desktop-webview

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/desktop-webview dev
```

## 构建

```bash
pnpm --filter @wae-example/desktop-webview build
```

## 覆盖

- @wae/client
- @wae/wae
- 当前桌面平台包
- WebView bridge
- 窗口 API 契约
- native capability 契约

## 不覆盖

- Cloudflare
- 移动安全区
