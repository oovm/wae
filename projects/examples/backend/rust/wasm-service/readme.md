# backend/rust/wasm-service

## 这个示例展示什么

Rust 编译为 Wasm 的服务占位：强调 Wasm 服务与「页面里的 `@wae/client`」不同。

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
调用方
  → Wasm 导出 / WASI 或 Host 嵌入
  → Rust wasm-service 逻辑
```

## 练习点

- 对照 `frontend/wasm`：那边是前端 opt-in；这边是服务侧 Wasm。
- 阅读平台 Wasm 包说明，勿假定本目录已有 `wasm-bindgen` 工程。

## 与生产应用的差异

生产 Rust 服务是独立部署单元；WAE host 是本地壳。二者都叫「后端」时务必写清。

依赖说明：无本目录依赖。不参与 examples 的 pnpm filter。
