# backend-http

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/backend-http dev
```

## 构建

```bash
pnpm --filter @wae-example/backend-http build
```

## 覆盖

- @wae/server
- 路由
- middleware
- 请求上下文
- 错误响应
- JSON

## 不覆盖

- 客户端 UI
- Cloudflare 绑定
