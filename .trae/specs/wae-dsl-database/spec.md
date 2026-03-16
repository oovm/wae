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
- 实现 RBQ-Engine