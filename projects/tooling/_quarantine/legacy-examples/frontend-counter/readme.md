# frontend-counter

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/frontend-counter dev
```

## 构建

```bash
pnpm --filter @wae-example/frontend-counter build
```

## 覆盖

- @wae/client
- JSX/TSX
- signal
- 事件
- DOM 更新
- Vite

## 不覆盖

- server
- SSR
- WebView native capability
