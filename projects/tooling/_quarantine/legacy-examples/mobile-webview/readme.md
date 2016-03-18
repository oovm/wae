# mobile-webview

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/mobile-webview dev
```

## 构建

```bash
pnpm --filter @wae-example/mobile-webview build
```

## 覆盖

- 移动 viewport 契约
- 安全区域
- 触摸
- 软键盘
- 返回键
- 挂起/恢复

## 不覆盖

- 当前不可伪装为已可运行产品

## 状态

**尚未可运行。** 仅保留目录、依赖与验证契约，禁止在文档或 CI 中伪装为已通过产品验收。
