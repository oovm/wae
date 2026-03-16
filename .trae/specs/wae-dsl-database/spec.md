# WAE DSL 数据库管理 - 产品需求文档

## Overview
- **Summary**: 将 WAE 项目的 YAML 数据库管理方式改为基于 DSL 的方式，借鉴 RBQ-Engine 的思想，使用 oak rbq 解析器和 YYKV 的 we-trust gateway。
- **Purpose**: 提供更简洁、直观的数据库模型定义方式，减少配置复杂度，提高开发效率。
- **Target Users**: WAE 项目的开发者和维护者。

## Goals
- 将 YAML 数据库配置替换为 DSL 格式的 .wae 文件
- 实现基于 RBQ 语法的数据库模型定义
- 集成 YYKV 的 we-trust gateway 进行解析
- 保持与现有功能的兼容性
- 更新相关文档

## Non-Goals (Out of Scope)
- 实现 RBQ-Engine 的全部功能
- 修改 WAE 项目的核心架构
- 改变现有的数据库连接和操作逻辑

## Background & Context
- 当前 WAE 项目使用 YAML 文件管理数据库 schema，配置繁琐且容易出错
- RBQ-Engine 提供了简洁的 DSL 语法，使数据库模型定义更加直观
- YYKV 项目中的 we-trust gateway 已经实现了 RBQ 解析器

## Functional Requirements
- **FR-1**: 支持使用 .wae 文件定义数据库模型
- **FR-2**: 实现 RBQ 语法解析，包括模型定义、字段类型、约束和关系
- **FR-3**: 集成 YYKV 的 we-trust gateway 进行解析
- **FR-4**: 保持与现有数据库连接和操作逻辑的兼容性
- **FR-5**: 支持多数据库配置

## Non-Functional Requirements
- **NFR-1**: 解析性能高效，不影响项目启动速度
- **NFR-2**: 错误提示清晰，便于开发者定位问题
- **NFR-3**: 代码结构清晰，易于维护和扩展

## Constraints
- **Technical**: 使用 Rust 语言实现，依赖 YYKV 的 we-trust gateway
- **Dependencies**: 需要集成 YYKV 的 we-trust gateway 组件

## Assumptions
- YYKV 的 we-trust gateway 提供了完整的 RBQ 解析功能
- 现有数据库连接和操作逻辑不需要修改

## Acceptance Criteria

### AC-1: DSL 语法支持
- **Given**: 开发者创建了 schemas/user.wae 文件
- **When**: 项目启动时解析该文件
- **Then**: 系统能够正确解析文件中的模型定义
- **Verification**: `programmatic`

### AC-2: 模型定义功能
- **Given**: 开发者在 .wae 文件中定义了 User 模型，包含 id、name、email 字段
- **When**: 系统解析该文件
- **Then**: 系统能够正确识别模型及其字段属性
- **Verification**: `programmatic`

### AC-3: 关系定义支持
- **Given**: 开发者在 .wae 文件中定义了 User 和 Post 模型，并建立了关联关系
- **When**: 系统解析该文件
- **Then**: 系统能够正确识别模型间的关系
- **Verification**: `programmatic`

### AC-4: 多数据库支持
- **Given**: 开发者定义了多个数据库配置
- **When**: 系统启动时
- **Then**: 系统能够正确加载所有数据库配置
- **Verification**: `programmatic`

### AC-5: 文档更新
- **Given**: 数据库管理方式变更
- **When**: 开发者查看文档
- **Then**: 文档中包含 DSL 数据库管理的使用说明
- **Verification**: `human-judgment`

## Open Questions
- [ ] 是否需要保留 YAML 配置的兼容性，以便平滑过渡？
- [ ] 如何处理现有的 YAML 配置文件？
- [ ] DSL 语法是否需要扩展以支持 WAE 特有的功能？