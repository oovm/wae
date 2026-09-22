# backend/rust/websocket

## 这个示例展示什么

Rust WebSocket 服务占位：文档提醒 WS 也可在 Rust 侧实现，但本目录无工程。

## 运行前提

本目录目前**只有 README**，没有 `Cargo.toml` / 源码，**不能** `cargo run`，也没有 npm `package.json`。

## 启动命令

无本目录启动命令。相关可检查的宿主 crate 例如：

```bash
cargo check -p wae-bridge
```


## 访问地址

无。

## 关键文件

- 仅本 README（待补工程）

## 请求 / 事件路径（目标语义）

```text
WS
  → Rust service
  → 帧 / 推送
```

## 练习点

- 对比 `backend/typescript/websocket`：语言与部署单元不同，语义都是「无 WAE 前端的后端」。
- 不要从本目录复制不存在的 `Cargo.toml`。

## 与生产应用的差异

生产 Rust 服务是独立部署单元；WAE host 是本地壳。二者都叫「后端」时务必写清。

依赖说明：无本目录依赖。不参与 examples 的 pnpm filter。
