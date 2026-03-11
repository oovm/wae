# WAE 完整示例集 - Product Requirement Document

## Overview
- **Summary**: 为 WAE 框架构建一套完整的、实用的示例项目集合，覆盖框架的所有核心功能和使用场景，让开发者能够像参考 Axum/Rocket 那样通过示例快速上手 WAE。
- **Purpose**: 解决当前只有一个基础 effect 示例的问题，提供丰富多样的示例来展示 WAE 框架的全部能力，提升框架的可用性和学习曲线。
- **Target Users**: WAE 框架的新用户、学习 Rust Web 开发的开发者、希望使用 WAE 构建微服务的团队。

## Goals
- 创建 15+ 个高质量、可运行的示例项目
- 覆盖 WAE 框架的所有核心模块功能
- 每个示例都有清晰的 README 文档说明功能和运行方式
- 所有示例都能正常编译和运行
- 将所有示例加入 Cargo Workspace

## Non-Goals (Out of Scope)
- 不创建生产级完整应用（只做演示用）
- 不添加复杂的前端界面
- 不编写详细的单元测试（示例代码本身即可）
- 不进行性能优化和安全审计

## Background & Context
WAE 是一个功能丰富的 Rust 微服务框架，包含了 HTTP 服务、数据库、认证、缓存、队列、调度等多个核心模块。目前项目中只有 `effect-basic` 一个示例，与 Axum/Rocket 等成熟框架丰富的示例相比差距较大。丰富的示例能够帮助开发者快速理解和使用框架的各项功能。

## Functional Requirements
- **FR-1**: 基础 HTTP 服务示例 - 展示路由、请求处理、响应格式化
- **FR-2**: 数据库操作示例 - 展示 Turso/PostgreSQL/MySQL 的连接和 CRUD 操作
- **FR-3**: 认证与授权示例 - 展示 JWT、OAuth2、密码认证等
- **FR-4**: 缓存示例 - 展示 Redis 缓存的使用
- **FR-5**: 队列与任务调度示例 - 展示任务队列和定时任务
- **FR-6**: 可观测性示例 - 展示健康检查、指标、日志、追踪
- **FR-7**: WebSocket 示例 - 展示实时通信功能
- **FR-8**: 中间件示例 - 展示 CORS、压缩、请求 ID、追踪等中间件
- **FR-9**: 文件上传与静态文件服务示例
- **FR-10**: 模板渲染示例 - 展示 HTML 模板功能
- **FR-11**: API 文档示例 - 展示 OpenAPI/Swagger UI 集成
- **FR-12**: 微服务示例 - 展示多服务协作
- **FR-13**: 错误处理示例 - 展示统一错误响应
- **FR-14**: 配置管理示例 - 展示配置加载
- **FR-15**: Effect 依赖注入进阶示例

## Non-Functional Requirements
- **NFR-1**: 每个示例都有独立的 Cargo.toml 和 README.md
- **NFR-2**: 所有示例代码都遵循项目现有代码风格
- **NFR-3**: 所有示例都能通过 `cargo check --all-features`
- **NFR-4**: 示例代码结构清晰，易于理解和学习

## Constraints
- **Technical**: 使用 Rust 2024 Edition，遵循现有代码风格
- **Business**: 需要在合理时间内完成
- **Dependencies**: 依赖于 WAE 现有各模块的功能

## Assumptions
- WAE 各核心模块的 API 相对稳定
- 示例主要用于演示，不需要处理极端边缘情况
- 开发者有基本的 Rust 和 Web 开发知识

## Acceptance Criteria

### AC-1: 完整示例覆盖
- **Given**: WAE 框架包含多个功能模块
- **When**: 完成所有规划的示例
- **Then**: 每个核心模块都有对应的示例展示其功能
- **Verification**: `programmatic` (检查示例目录数量和覆盖的模块)

### AC-2: 示例可运行
- **Given**: 已创建所有示例
- **When**: 执行 `cargo run --example X`
- **Then**: 示例能够正常编译和启动
- **Verification**: `programmatic`

### AC-3: 文档完整
- **Given**: 每个示例项目
- **When**: 查看示例目录
- **Then**: 每个示例都有 README.md 说明功能和运行方式
- **Verification**: `human-judgment`

### AC-4: Workspace 集成
- **Given**: 根目录 Cargo.toml
- **When**: 查看 workspace members
- **Then**: 所有示例都已加入 workspace
- **Verification**: `programmatic`

### AC-5: 代码质量
- **Given**: 所有示例代码
- **When**: 运行 `cargo check --all-features`
- **Then**: 无编译错误和警告
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要创建一个综合示例来展示多个模块的协作？
- [ ] 示例是否需要包含测试用例？
