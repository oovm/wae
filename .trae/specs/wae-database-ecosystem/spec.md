# wae-database 生态完善 - Product Requirement Document

## Overview
- **Summary**: 完善 wae-database 模块的生态系统，包括与 HTTP 框架的集成、完整的自动迁移工具实现以及异步连接池优化
- **Purpose**: 解决当前 wae-database 模块与 HTTP 框架集成不足、自动迁移工具不完整、连接池性能有待优化的问题，使 wae-database 成为一个生产就绪的数据库 ORM 解决方案
- **Target Users**: 使用 wae 框架构建 Web 应用的开发者

## Goals
- 提供数据库连接池提取器，用于从 HTTP 请求中获取数据库连接
- 实现事务中间件，支持自动事务管理和回滚
- 完善 auto-migrate 功能，使其具备完整的数据库迁移能力
- 优化异步连接池实现，提升性能和可靠性
- 确保所有公共 API 都有完整的文档注释

## Non-Goals (Out of Scope)
- 不实现新的数据库驱动支持
- 不重构现有的 ORM 查询构建器
- 不改变现有的数据库连接 API 接口

## Background & Context
- wae-database 模块目前提供了基本的数据库抽象层和 ORM 功能
- 现有的 wae-https 模块有 extract 和 middleware 模块，可以进行集成
- wae-tools 模块已有 auto-migrate 的框架，但实现不完整
- 目前使用 deadpool-postgres 和其他连接池实现，但深度优化不足

## Functional Requirements
- **FR-1**: 提供数据库连接提取器，支持从 HTTP 请求中获取数据库连接
- **FR-2**: 实现事务中间件，支持请求级别的自动事务管理
- **FR-3**: 完善 auto-migrate 功能，支持完整的数据库迁移流程
- **FR-4**: 实现数据库连接池的深度优化配置
- **FR-5**: 提供迁移版本管理功能

## Non-Functional Requirements
- **NFR-1**: 连接池提取器性能开销小于 1ms
- **NFR-2**: 事务中间件在事务失败时能够正确回滚
- **NFR-3**: auto-migrate 操作应在 10 秒内完成（针对中等规模数据库）
- **NFR-4**: 连接池优化后，连接获取时间减少 30%
- **NFR-5**: 所有公共结构体、枚举、方法、字段都有完整的文档注释

## Constraints
- **Technical**: 必须使用 Rust 和现有 wae 框架的架构
- **Business**: 保持与现有代码库的兼容性
- **Dependencies**: 依赖现有的 wae-database、wae-https、wae-tools 模块

## Assumptions
- 现有的 wae-https extract 和 middleware 架构可以支持新的集成
- auto-migrate 的现有框架可以扩展为完整功能
- 现有的连接池实现（deadpool-postgres 等）可以进行深度优化

## Acceptance Criteria

### AC-1: 数据库连接提取器
- **Given**: HTTP 框架已配置数据库连接池
- **When**: 处理 HTTP 请求时使用数据库连接提取器
- **Then**: 可以从请求中获取可用的数据库连接
- **Verification**: `programmatic`
- **Notes**: 应支持多种数据库后端（PostgreSQL、MySQL、Turso）

### AC-2: 事务中间件
- **Given**: 路由已配置事务中间件
- **When**: 请求处理过程中发生错误
- **Then**: 事务自动回滚；请求成功则自动提交
- **Verification**: `programmatic`

### AC-3: 完整的 auto-migrate
- **Given**: 有实体定义和现有数据库
- **When**: 运行 auto-migrate
- **Then**: 数据库 schema 与实体定义保持一致，包括表创建、列修改、索引管理
- **Verification**: `programmatic`

### AC-4: 连接池优化配置
- **Given**: 数据库连接池已初始化
- **When**: 配置连接池参数（最大连接数、超时、回收策略等）
- **Then**: 连接池按预期工作，性能得到优化
- **Verification**: `programmatic`

### AC-5: 文档注释完整性
- **Given**: 所有新增的公共 API
- **When**: 检查代码文档
- **Then**: 所有公共结构体、枚举、方法、字段都有完整的文档注释，不使用后置注释
- **Verification**: `human-judgment`

## Open Questions
- [ ] 事务隔离级别是否需要可配置？
- [ ] auto-migrate 是否需要支持向下迁移（rollback）？
- [ ] 连接池是否需要支持健康检查？
