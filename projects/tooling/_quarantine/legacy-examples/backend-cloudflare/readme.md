# backend-cloudflare

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/backend-cloudflare dev
```

## 构建

```bash
pnpm --filter @wae-example/backend-cloudflare build
```

## 覆盖

- fetch handler
- Env bindings
- KV adapter
- waitUntil
- 流式响应
- 错误处理

## 不覆盖

- 桌面 WebView
- 长驻 Node 文件系统
