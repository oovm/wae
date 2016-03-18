# frontend-storage

## 用途

产品验证矩阵条目：见下方覆盖范围。

## 运行

```bash
pnpm --filter @wae-example/frontend-storage dev
```

## 构建

```bash
pnpm --filter @wae-example/frontend-storage build
```

## 覆盖

- 存储 capability adapter
- 离线状态
- 资源缓存抽象

## 不覆盖

- 直接依赖浏览器专属 API 作为业务入口
- server
