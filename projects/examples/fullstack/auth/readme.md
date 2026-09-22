# @wae-example/fullstack-auth

## 这个示例展示什么

全栈鉴权意图：client + server 同仓占位，后续应挂 session / cookie / token 相关 handler。当前源码与其它 fullstack 一样只是双构造。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/fullstack-auth run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts` — `createClient` + `createServer`
- `package.json`

## 请求 / 事件路径（目标语义）

```text
登录表单 / client
  → POST /auth/…（目标）
  → server 校验 → Set-Cookie / token
  → 后续请求带凭证 → 受保护路由
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 设计一条公开 `route` 与一条需鉴权的 `route`，先用 `app.fetch` + 手写 Header 模拟。
- 阅读 `@wae/client` session 相关类型（若有），不要假设已有完整 OAuth。
- 与 `fullstack/rpc` 对比：鉴权是横切，不是另一种 transport。

## 与生产应用的差异

生产有真实 IdP、密钥轮换与 CSRF；本示例无登录页、无 cookie 实现。

依赖（本示例）：`@wae/client`、`@wae/server`、`@wae/serverless`。
