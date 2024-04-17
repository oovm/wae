# 简介

WAE (Rust Async Utilities) 是一个微服务优先的全栈异步框架，提供 Rust 后端和 TypeScript 前端的一站式解决方案。

## 为什么选择 WAE？

### 全栈解决方案

WAE 提供完整的全栈开发能力：

| 层级 | Rust 后端 | TypeScript 前端 |
|-----|----------|----------------|
| HTTP 服务 | wae-https (基于 axum) | @wae/client |
| 认证服务 | wae-authentication | @wae/auth |
| WebSocket | wae-websocket | @wae/websocket |
| 对象存储 | wae-storage | @wae/storage |
| 类型定义 | wae-types | @wae/core |

### 微服务优先

WAE 从设计之初就以微服务架构为核心考量：

- **服务发现**: 内置服务注册与发现机制 (wae-service)
- **配置中心**: 统一的配置管理，支持热更新 (wae-config)
- **弹性容错**: 熔断器、限流器、重试、超时 (wae-resilience)
- **任务调度**: 定时任务、延迟任务 (wae-scheduler)
- **健康检查**: 标准化的健康检查端点

### 深度融合 tokio

WAE 与 tokio 运行时深度绑定，充分发挥异步优势：

- **原生 async/await**: 所有 API 都是异步优先
- **tokio 任务调度**: 自动利用多核并行
- **零拷贝优化**: 最小化数据复制
- **背压支持**: 原生支持流式处理

### 前后端类型同步

WAE 提供完整的 TypeScript 类型定义，与 Rust 后端类型一一对应：

```typescript
// 前端 TypeScript
import { CloudError, ApiResponse, UserInfo } from "@wae/core";

// 后端 Rust
pub struct UserInfo {
    pub id: UserId,
    pub username: String,
    // ...
}
```

## 模块概览

### 后端模块 (Rust)

| 模块 | 说明 |
|-----|------|
| wae-types | 核心类型定义 |
| wae-https | HTTP/HTTPS 服务 |
| wae-authentication | 认证服务 (JWT, OAuth2, SAML, TOTP) |
| wae-storage | 对象存储 (COS, OSS, Local) |
| wae-ai | AI 服务抽象 |
| wae-websocket | WebSocket 服务 |
| wae-service | 服务发现与注册 |
| wae-config | 配置管理 |
| wae-resilience | 弹性容错 |
| wae-scheduler | 任务调度 |
| wae-session | Session 管理 |
| wae-email | 邮件服务 |
| wae-database | 数据库 ORM |
| wae-event | 事件驱动 |
| wae-queue | 消息队列 |
| wae-testing | 测试支持 |

### 前端模块 (TypeScript)

| 模块 | 说明 |
|-----|------|
| @wae/core | 核心类型定义 |
| @wae/client | HTTP 客户端 |
| @wae/auth | 认证客户端 |
| @wae/websocket | WebSocket 客户端 |
| @wae/storage | 存储服务客户端 |

## 下一步

- [快速开始](/guide/getting-started) - 5 分钟上手 WAE
- [核心优势](/guide/advantages) - 了解 WAE 的设计理念
- [架构设计](/architecture/overview) - 深入了解 WAE 的架构
