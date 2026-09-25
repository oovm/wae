# backend/rust/http

## 这个示例展示什么

Rust HTTP 服务占位目录：说明「独立 Rust HTTP 服务」与「WAE TS `@wae/server`」不是同一层。

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
HTTP
  → Rust service（非 @wae/server）
  → Response
```

## 练习点

- 读 [`projects/crates`](../../../../crates/readme.md) 与 [`@wae/server`](../../../../packages/server/readme.md)，写清二者职责边界。
- 可 `cargo check -p wae-bridge` 熟悉仓库 Rust 侧，但那是 host，不是本目录服务。

## 与生产应用的差异

生产 Rust 服务是独立部署单元；WAE host 是本地壳。二者都叫「后端」时务必写清。

依赖说明：无本目录依赖。不参与 examples 的 pnpm filter。
