# backend-ssr

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/backend-ssr dev
```

## 构建

```bash
pnpm --filter @wae-example/backend-ssr build
```

## 覆盖

- 服务端 View 渲染
- HTML 输出
- hydration metadata
- 初始状态注入
- 客户端接管

## 不覆盖

- Cloudflare DO
- 桌面 native
