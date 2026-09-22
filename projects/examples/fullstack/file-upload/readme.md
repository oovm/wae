# @wae-example/fullstack-file-upload

## 这个示例展示什么

文件上传意图：client 提交 multipart/body，server 接收并响应。源码未处理 `FormData` 或磁盘写入。

## 运行前提

- 仓库根已 `pnpm install`
- Node.js 22 + pnpm 10（与本仓 `packageManager` 一致）
- 骨架：`src/main.ts` 仅构造 API，无 HTTP 监听、无浏览器页、无 UI。


## 启动命令

在仓库根：

```bash
pnpm --filter @wae-example/fullstack-file-upload run check
```

进入本目录亦可：

```bash
pnpm run check
pnpm run build   # 当前打印 skeleton，不产出可部署包
```

## 访问地址

**无。** 没有 localhost 端口，也没有可打开的静态页。今天能验收的只有 `pnpm run check`（`tsc --noEmit`）。

## 关键文件

- `src/main.ts`
- `package.json`

## 请求 / 事件路径（目标语义）

```text
client FormData / body
  → POST /upload（目标）
  → createServer handler 读 body
  → 存储或回显元数据 → JSON Response
```

0.0.0 代码通常只停在「构造对象」一步，尚未把整条路径跑通。

## 练习点

- 写一个读 `request.arrayBuffer()` 或 `formData()` 的 `route`，用 `app.fetch` 喂 `new Request(..., { method:"POST", body })`。
- 限制体积极限与 MIME——哪怕只在注释里写清。
- 对比 `fullstack/rpc`：上传关心 body 形态，不只是 JSON action。

## 与生产应用的差异

生产有对象存储、病毒扫描与断点续传；本示例无文件落盘。

依赖（本示例）：`@wae/client`、`@wae/server`、`@wae/serverless`。
