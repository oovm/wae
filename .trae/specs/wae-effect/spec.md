# wae-effect 依赖注入/效果系统增强 - Product Requirement Document

## Overview
- **Summary**: 增强 wae-effect 模块，从简单的字符串键值对容器升级为功能完整的类型安全依赖注入系统，支持生命周期管理、作用域和自动注入。
- **Purpose**: 解决当前依赖注入缺乏类型安全、生命周期管理、作用域控制等问题，提供更强大、更易用的依赖管理能力。
- **Target Users**: WAE 框架的开发者，使用 wae-effect 进行依赖管理的应用开发人员。

## Goals
- 提供编译时类型安全的依赖解析
- 支持依赖的生命周期管理
- 实现多种作用域（Singleton, Request-scoped 等）
- 提供自动依赖注入能力
- 在 wae-macros 中添加 use_xxx!() 宏以简化使用

## Non-Goals (Out of Scope)
- 不实现完整的依赖图自动解析（如循环依赖检测）
- 不提供依赖的延迟初始化/懒加载
- 不改变现有 API 的向后兼容性（现有代码应能继续工作）

## Background & Context
- 当前 `wae-effect` 实现仅为基于字符串键的 `HashMap<Box<dyn Any>>` 容器
- 现有实现缺乏类型安全，依赖获取需要手动指定字符串键
- 文档中已规划了 use_xxx!() 宏等便捷功能，但尚未实现
- 项目整体使用 Rust 语言，基于 Axum 构建 HTTP 服务

## Functional Requirements
- **FR-1**: 类型安全的依赖注册和获取（基于 TypeId，无需字符串键）
- **FR-2**: 支持 Singleton 作用域（整个应用生命周期唯一实例）
- **FR-3**: 支持 Request-scoped 作用域（每个请求唯一实例）
- **FR-4**: 在 wae-macros 中提供 use_xxx!() 宏
- **FR-5**: 保留向后兼容性（现有 API 继续可用）

## Non-Functional Requirements
- **NFR-1**: 编译时类型检查（避免运行时类型错误）
- **NFR-2**: 性能影响可忽略（依赖注入 overhead < 1%）
- **NFR-3**: 完整的文档注释（所有 public API 必须有文档）

## Constraints
- **Technical**: Rust 语言，保持与现有代码库风格一致，使用 Arc 管理共享状态
- **Business**: 不引入新的外部依赖
- **Dependencies**: wae-types（用于 WaeError），wae-macros（用于宏实现）

## Assumptions
- 现有用户主要使用 Singleton 模式的依赖
- Request-scoped 依赖主要用于 HTTP 请求处理
- 向后兼容性是重要的考虑因素

## Acceptance Criteria

### AC-1: 类型安全的依赖注册和获取
- **Given**: 有一个依赖类型 MyService
- **When**: 通过类型安全的方式注册和获取该依赖
- **Then**: 编译时验证类型正确性，运行时成功获取到正确类型的依赖实例
- **Verification**: `programmatic`

### AC-2: Singleton 作用域
- **Given**: 一个 Singleton 作用域的依赖
- **When**: 在不同请求/上下文中多次获取该依赖
- **Then**: 始终返回同一个实例
- **Verification**: `programmatic`

### AC-3: Request-scoped 作用域
- **Given**: 一个 Request-scoped 作用域的依赖
- **When**: 在不同请求中获取该依赖
- **Then**: 每个请求返回独立的实例
- **Verification**: `programmatic`

### AC-4: use_xxx!() 宏可用
- **Given**: 已注册多个依赖
- **When**: 使用 use_xxx!() 宏在 Effectful 上下文中获取依赖
- **Then**: 宏能正确展开，编译通过，运行时成功获取依赖
- **Verification**: `programmatic`

### AC-5: 向后兼容性
- **Given**: 现有使用旧 API 的代码
- **When**: 使用新的增强版 wae-effect
- **Then**: 现有代码无需修改即可正常编译和运行
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要支持更多作用域类型（如 Session-scoped）？
- [ ] 是否需要依赖图自动解析和自动注入？
