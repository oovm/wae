# wae-https 核心功能 - 产品需求文档

## Overview
- **Summary**: 本项目实现 wae-https 模块的核心功能，包括完整的路由系统、路径/查询/请求体提取器、请求分发匹配、中间件支持、静态文件服务和服务器启动能力。
- **Purpose**: 解决 wae-https 当前 Router 为空实现、serve_plain 和 serve_tls 无限睡眠的问题，提供与 Axum/Rocket 成熟度相当的 HTTP 框架核心功能。
- **Target Users**: 使用 wae 框架的开发者，需要构建高性能 HTTP/HTTPS Web 应用的团队。

## Goals
- 实现完整的路由系统，支持多种 HTTP 方法和嵌套路由
- 实现路径参数、查询参数、JSON/Form 等请求体提取器
- 实现请求分发匹配机制
- 集成 Tower 生态中间件系统
- 实现静态文件服务能力
- 修复 serve_plain 和 serve_tls 启动功能
- 保持与现有 wae-https 模块接口的兼容性

## Non-Goals (Out of Scope)
- 完全重构 wae-https 的整体架构
- 新增与现有需求无关的功能
- 重写已实现的中间件（compression, cors 等）

## Background & Context
- wae-https 是 wae 框架的核心模块，负责提供 HTTP/HTTPS 服务能力
- 当前状态：Router 是空结构体，路由系统为占位实现，serve_plain 和 serve_tls 只是无限循环
- 项目已有成熟的依赖库支持：hyper、hyper-util、tower、tower-http、matchit 等
- 需要保持与现有 API 接口的兼容性

## Functional Requirements
- **FR-1**: 完整的路由系统，支持 GET、POST、PUT、DELETE、PATCH、OPTIONS、HEAD、TRACE 方法
- **FR-2**: 路径参数提取能力（如 /user/{id}）
- **FR-3**: 查询参数提取能力
- **FR-4**: JSON 请求体提取能力
- **FR-5**: Form 请求体提取能力
- **FR-6**: 请求分发匹配机制
- **FR-7**: 中间件系统集成（支持 Tower 生态）
- **FR-8**: 静态文件服务支持
- **FR-9**: 实现 serve_plain 和 serve_tls 真实服务器启动
- **FR-10**: 合并路由和嵌套路由功能

## Non-Functional Requirements
- **NFR-1**: 性能与 Axum 相当
- **NFR-2**: 良好的文档注释（所有公共结构体、枚举、方法、字段）
- **NFR-3**: 与现有 wae 生态集成良好
- **NFR-4**: 遵循 Rust 安全最佳实践

## Constraints
- **Technical**: 使用 Rust，必须保持与现有依赖库（hyper、tower 等）的兼容性
- **Business**: 保持现有 API 接口不变，不破坏向后兼容性
- **Dependencies**: 依赖现有 crates.io 中的库，不引入新的大型依赖

## Assumptions
- 可以使用 hyper-util 中的服务构建工具
- 可以使用 matchit 作为路由匹配引擎
- 可以使用 tower-http 提供的静态文件服务功能
- 现有的提取器类型定义（Path、Query、Json 等）可以保留

## Acceptance Criteria

### AC-1: 路由系统功能完整
- **Given**: 有一个 Router 实例
- **When**: 使用各种 HTTP 方法和路径添加路由
- **Then**: 路由能够正确匹配和处理请求
- **Verification**: `programmatic`

### AC-2: 路径参数提取正常工作
- **Given**: 有一个带路径参数的路由（如 /user/{id}）
- **When**: 发送匹配路径的请求
- **Then**: 路径参数能正确提取并传递给处理函数
- **Verification**: `programmatic`

### AC-3: 查询参数提取正常工作
- **Given**: 有一个处理查询参数的路由
- **When**: 发送带查询参数的请求
- **Then**: 查询参数能正确提取并反序列化
- **Verification**: `programmatic`

### AC-4: JSON 请求体提取正常工作
- **Given**: 有一个接受 JSON 请求体的路由
- **When**: 发送带 JSON 数据的请求
- **Then**: JSON 数据能正确提取并反序列化
- **Verification**: `programmatic`

### AC-5: Form 请求体提取正常工作
- **Given**: 有一个接受 Form 请求体的路由
- **When**: 发送带 Form 数据的请求
- **Then**: Form 数据能正确提取并反序列化
- **Verification**: `programmatic`

### AC-6: 中间件能够正常工作
- **Given**: 有一个配置了中间件的路由
- **When**: 发送请求到该路由
- **Then**: 中间件能够正确处理请求和响应
- **Verification**: `programmatic`

### AC-7: 静态文件服务能够正常工作
- **Given**: 配置了静态文件服务
- **When**: 请求静态文件
- **Then**: 能够正确返回文件内容和正确的 MIME 类型
- **Verification**: `programmatic`

### AC-8: 服务器能够正常启动
- **Given**: 配置了 HttpsServer
- **When**: 调用 serve() 方法
- **Then**: 服务器能够正常监听端口并处理请求
- **Verification**: `programmatic`

### AC-9: API 保持向后兼容性
- **Given**: 现有使用 wae-https 的代码
- **When**: 升级到新实现
- **Then**: 现有代码能够无修改编译和运行
- **Verification**: `programmatic`

### AC-10: 完整的文档注释
- **Given**: 新实现的代码
- **When**: 检查所有公共 API
- **Then**: 所有公共结构体、枚举、方法、字段都有文档注释，无后置注释
- **Verification**: `human-judgment`

## Open Questions
- 无
