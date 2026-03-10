# WAE - Wae Algebraic Effects

**微服务优先的 Rust 异步框架，深度融合 tokio，提供从 HTTP 服务到云服务的一站式解决方案。**

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub](https://img.shields.io/badge/github-oovm/wae-blue.svg)](https://github.com/oovm/wae)

---

## ✨ 核心特性

| 特性               | 说明                                   |
|------------------|--------------------------------------|
| 🚀 **微服务优先**     | 服务发现、配置中心、链路追踪、健康检查、优雅关闭，开箱即用        |
| ⚡ **深度融合 tokio** | 基于 tokio-net 零抽象，原生 async/await，极致性能 |
| 🤖 **AI 友好**     | 清晰的 Trait 抽象，统一的错误处理，最小化样板代码         |
| 🦀 **纯血 Rust**   | 零 FFI 绑定，跨平台支持，内存安全，可审计              |

---

## 📦 模块概览

WAE 采用模块化的 Crate 设计，每个模块可独立使用：

| 模块               | 功能                            |
|------------------|-------------------------------|
| **wae-types**    | 核心类型定义                     |
| **wae-https**    | HTTP/HTTPS 服务，构建器模式，统一响应结构    |
| **wae-effect**   | 代数效应依赖注入，声明式依赖获取              |
| **wae-ai**       | AI 服务抽象，支持腾讯混元、火山引擎等          |
| **wae-storage**  | 存储服务抽象，支持腾讯云 COS、阿里云 OSS、本地存储 |
| **wae-database** | 数据库 ORM，基于 Turso (SQLite 兼容)    |
| **wae-cache**    | 缓存服务抽象，支持内存缓存                 |
| **wae-service**  | 服务发现与注册，负载均衡策略                |
| **wae-config**   | 多层级配置管理，支持 TOML/YAML/环境变量     |
| **wae-email**    | 邮件服务抽象，支持 SMTP/Sendmail       |

---

## 🚀 快速开始

### 安装

```toml
[dependencies]
wae = { git = "https://github.com/oovm/wae" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

### 基础 HTTP 服务

```rust
use wae::prelude::*;

#[wae::main]
async fn main() -> Result<()> {
    Server::bind("0.0.0.0:3000")
        .routes(routes())
        .with_graceful_shutdown()
        .serve()
        .await
}
```

更多详细示例请查看 [快速开始文档](documentation/zh-hans/guide/getting-started.md)。

---

## 📖 文档

- [简介](documentation/zh-hans/guide/introduction.md) - 了解 WAE 的设计理念
- [快速开始](documentation/zh-hans/guide/getting-started.md) - 5 分钟上手 WAE
- [核心优势](documentation/zh-hans/guide/advantages.md) - 深入了解各模块特性
- [架构设计](documentation/zh-hans/architecture/overview.md) - 模块化设计原则

---

## 📝 开发规范

### Emoji 提交规范

| Emoji  | 含义      |
|--------|---------|
| 🎂     | 项目初始化   |
| 🎉     | 发布新版本   |
| 🧪🔮   | 实验性代码   |
| 🔧🐛🐞 | Bug 修复  |
| 🔒     | 安全修复    |
| 🐣🐤🐥 | 新增功能    |
| 📝🎀   | 文档更新    |
| 🚀     | 性能优化    |
| 🚧     | 开发中     |
| 🚨     | 测试覆盖    |
| 🚥     | CI 改进   |
| 🔥🧨   | 删除代码或文件 |
| 🧹     | 代码重构    |
| 📈     | 添加分析或分支 |
| 🤖     | 自动化修复   |
| 📦     | 更新依赖    |

---

## 📄 License

MIT License
