# WAE 数据库替换 - Turso 改为 Limbo - 产品需求文档

## Overview
- **Summary**: 将 WAE 框架中的 Turso 数据库实现替换为 Limbo 数据库实现，保持 API 兼容性的同时提升性能和可靠性。
- **Purpose**: 替换依赖的数据库后端，从 Turso 迁移到 Limbo，以获得更好的性能和维护性。
- **Target Users**: WAE 框架的开发者和使用者。

## Goals
- 移除 Turso 依赖，替换为 Limbo 依赖
- 保持数据库 API 接口不变，确保向后兼容性
- 实现与 Turso 相同的功能特性
- 确保所有现有测试通过

## Non-Goals (Out of Scope)
- 不修改其他数据库后端实现（PostgreSQL、MySQL）
- 不更改数据库 API 设计
- 不添加新的数据库功能

## Background & Context
- 当前 WAE 框架使用 Turso 作为 SQLite 的实现
- 需要将 Turso 替换为 Limbo 以提升性能和维护性
- 替换过程需要保持 API 兼容性，确保现有代码不受影响

## Functional Requirements
- **FR-1**: 移除 Turso 依赖，添加 Limbo 依赖
- **FR-2**: 实现 Limbo 数据库连接和服务
- **FR-3**: 保持数据库 API 接口不变
- **FR-4**: 支持内存数据库和文件数据库
- **FR-5**: 支持事务操作
- **FR-6**: 支持参数化查询

## Non-Functional Requirements
- **NFR-1**: 性能不低于 Turso 实现
- **NFR-2**: 代码质量符合 WAE 框架标准
- **NFR-3**: 所有现有测试通过

## Constraints
- **Technical**: 保持 API 兼容性，不破坏现有代码
- **Dependencies**: 替换 Turso 为 Limbo 依赖

## Assumptions
- Limbo 提供与 Turso 类似的 API
- 替换过程中可以保持相同的功能特性

## Acceptance Criteria

### AC-1: Turso 依赖已移除
- **Given**: 项目构建配置
- **When**: 检查 Cargo.toml 文件
- **Then**: 不存在 Turso 依赖，存在 Limbo 依赖
- **Verification**: `programmatic`

### AC-2: Limbo 实现完成
- **Given**: 代码库
- **When**: 检查 wae-database 模块
- **Then**: 存在 Limbo 相关实现文件
- **Verification**: `programmatic`

### AC-3: API 接口保持不变
- **Given**: 现有代码
- **When**: 使用相同的 API 调用
- **Then**: 代码能够正常编译和运行
- **Verification**: `programmatic`

### AC-4: 功能特性完整
- **Given**: 测试用例
- **When**: 运行测试
- **Then**: 所有测试通过
- **Verification**: `programmatic`

## Open Questions
- [ ] Limbo 的具体 API 与 Turso 有哪些差异需要适配？
- [ ] 是否需要修改现有的测试用例？
- [ ] Limbo 的性能表现如何？