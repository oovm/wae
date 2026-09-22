# backend/rust/native-service

## 这个示例展示什么

面向本机能力的 Rust native service 占位（文件、设备等），与 WebView host 相关但不是 TS server。

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
前端 / 其它进程
  → IPC 或本地协议
  → Rust native service
  → 系统调用
```

## 练习点

- 对照 `native/filesystem` 等 TS 示例：页面侧意图 vs 本目录「服务进程」意图。
- 真正可测的桥在 `wae-bridge` / host crates。

## 与生产应用的差异

生产 Rust 服务是独立部署单元；WAE host 是本地壳。二者都叫「后端」时务必写清。

依赖说明：无本目录依赖。不参与 examples 的 pnpm filter。
