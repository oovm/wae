# WAE 完整示例集 - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: 基础 HTTP 服务示例 (https-basic)
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 创建基础 HTTP 服务示例
  - 展示路由、请求处理、响应格式化
  - 包含 GET/POST/PUT/DELETE 等常用 HTTP 方法
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-1.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-1.2: 示例能够正常编译和启动
  - `human-judgement` TR-1.3: 有清晰的 README.md 文档
- **Notes**: 作为第一个示例，注意代码风格和结构的一致性

## [x] Task 2: 数据库操作示例 (database-basic)
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 创建数据库操作示例
  - 展示 Turso 数据库连接和 CRUD 操作
  - 使用 ORM 功能
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-2.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-2.2: 示例能够正常编译
  - `human-judgement` TR-2.3: 有清晰的 README.md 文档
- **Notes**: 使用内存数据库或本地 SQLite 便于运行

## [x] Task 3: 认证与授权示例 (auth-basic)
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 创建认证与授权示例
  - 展示 JWT、密码认证、用户管理
  - 使用内存认证服务
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-3.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-3.2: 示例能够正常编译
  - `human-judgement` TR-3.3: 有清晰的 README.md 文档
- **Notes**: 展示登录、注册、Token 验证流程

## [x] Task 4: 缓存示例 (cache-basic)
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 创建缓存示例
  - 展示 Redis 缓存连接和基本操作
  - 展示缓存策略
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-4.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-4.2: 示例能够正常编译
  - `human-judgement` TR-4.3: 有清晰的 README.md 文档
- **Notes**: 说明如何运行 Redis 或使用 Mock

## [ ] Task 5: 队列与任务调度示例 (queue-scheduler)
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 创建队列与任务调度示例
  - 展示任务队列和定时任务
  - 使用 wae-scheduler 和 wae-queue
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-5.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-5.2: 示例能够正常编译
  - `human-judgement` TR-5.3: 有清晰的 README.md 文档
- **Notes**: 展示 cron 任务、间隔任务、延迟任务

## [x] Task 6: 可观测性示例 (observability-basic)
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 创建可观测性示例
  - 展示健康检查、指标、日志、追踪
  - 使用 wae-observability
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-6.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-6.2: 示例能够正常编译
  - `human-judgement` TR-6.3: 有清晰的 README.md 文档
- **Notes**: 展示健康检查端点、指标收集、JSON 日志

## [ ] Task 7: 中间件示例 (middleware-basic)
- **Priority**: P1
- **Depends On**: Task 1
- **Description**: 
  - 创建中间件示例
  - 展示 CORS、压缩、请求 ID、追踪等中间件
  - 基于 wae-https 的中间件
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-7.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-7.2: 示例能够正常编译
  - `human-judgement` TR-7.3: 有清晰的 README.md 文档
- **Notes**: 展示如何组合多个中间件

## [ ] Task 8: 模板渲染示例 (template-basic)
- **Priority**: P1
- **Depends On**: Task 1
- **Description**: 
  - 创建模板渲染示例
  - 展示 HTML 模板功能
  - 使用 wae-https 的模板模块
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-8.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-8.2: 示例能够正常编译
  - `human-judgement` TR-8.3: 有清晰的 README.md 文档
- **Notes**: 展示动态数据渲染、模板继承

## [x] Task 9: 错误处理示例 (error-handling)
- **Priority**: P1
- **Depends On**: Task 1
- **Description**: 
  - 创建错误处理示例
  - 展示统一错误响应
  - 使用 WaeError 和 ApiResponse
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-9.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-9.2: 示例能够正常编译
  - `human-judgement` TR-9.3: 有清晰的 README.md 文档
- **Notes**: 展示不同类型的错误和对应的 HTTP 状态码

## [x] Task 10: 配置管理示例 (config-basic)
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 创建配置管理示例
  - 展示配置加载
  - 使用 wae-config
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-10.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-10.2: 示例能够正常编译
  - `human-judgement` TR-10.3: 有清晰的 README.md 文档
- **Notes**: 展示 TOML/YAML 配置文件加载、环境变量覆盖

## [x] Task 11: Effect 依赖注入进阶示例 (effect-advanced)
- **Priority**: P1
- **Depends On**: None
- **Description**: 
  - 创建 Effect 依赖注入进阶示例
  - 展示与 HTTP 服务集成
  - 展示类型安全的依赖获取
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-11.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-11.2: 示例能够正常编译
  - `human-judgement` TR-11.3: 有清晰的 README.md 文档
- **Notes**: 展示在 HTTP 处理函数中使用 Effect

## [ ] Task 12: API 文档示例 (openapi-basic)
- **Priority**: P2
- **Depends On**: Task 1
- **Description**: 
  - 创建 API 文档示例
  - 展示 OpenAPI/Swagger UI 集成
  - 使用 wae-schema
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-12.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-12.2: 示例能够正常编译
  - `human-judgement` TR-12.3: 有清晰的 README.md 文档
- **Notes**: 展示自动生成 API 文档

## [ ] Task 13: WebSocket 示例 (websocket-basic)
- **Priority**: P2
- **Depends On**: Task 1
- **Description**: 
  - 创建 WebSocket 示例
  - 展示实时通信功能
  - 使用 wae-websocket
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-13.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-13.2: 示例能够正常编译
  - `human-judgement` TR-13.3: 有清晰的 README.md 文档
- **Notes**: 展示简单的聊天应用

## [ ] Task 14: 文件上传与静态文件服务示例 (static-files)
- **Priority**: P2
- **Depends On**: Task 1
- **Description**: 
  - 创建文件上传与静态文件服务示例
  - 展示文件上传处理
  - 展示静态文件服务
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-14.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-14.2: 示例能够正常编译
  - `human-judgement` TR-14.3: 有清晰的 README.md 文档
- **Notes**: 展示 multipart 表单处理

## [ ] Task 15: 微服务示例 (microservice-basic)
- **Priority**: P2
- **Depends On**: Task 1, Task 2, Task 3
- **Description**: 
  - 创建微服务示例
  - 展示多服务协作
  - 使用 wae-service 进行服务发现
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-15.1: 示例能够通过 `cargo check --all-features`
  - `programmatic` TR-15.2: 示例能够正常编译
  - `human-judgement` TR-15.3: 有清晰的 README.md 文档
- **Notes**: 展示简单的用户服务和订单服务

## [x] Task 16: 更新 Cargo Workspace
- **Priority**: P0
- **Depends On**: Task 1-15
- **Description**: 
  - 将所有示例加入 Cargo Workspace
  - 更新根目录 Cargo.toml
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `programmatic` TR-16.1: `cargo check --workspace` 成功通过
- **Notes**: 确保所有示例都在 workspace members 中
