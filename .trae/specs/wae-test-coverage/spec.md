# WAE 测试覆盖与稳定性 - 产品需求文档

## Overview
- **Summary**: 为 WAE 项目建立完整的测试体系，包括单元测试覆盖、集成测试、基准测试和性能测试，提升代码质量和稳定性。
- **Purpose**: 解决当前测试覆盖不足、缺少集成和基准测试的问题，构建可持续的测试基础设施。
- **Target Users**: WAE 项目的开发者和维护者。

## Goals
- 为所有后端模块提供单元测试覆盖
- 建立集成测试框架
- 添加基准测试和性能测试
- 改进 CI/CD 流程以支持测试覆盖率报告

## Non-Goals (Out of Scope)
- 不涉及前端代码的测试
- 不添加 E2E（端到端）测试
- 不重构现有生产代码，仅添加测试
- 不添加新的功能特性

## Background & Context
- WAE 是一个采用 workspace 结构的 Rust 项目，包含多个后端模块
- 项目已有 `wae-testing` 模块提供测试工具支持
- CI 已配置基本的测试运行
- 大多数模块的 tests/main.rs 文件基本为空
- 只有少数模块（如 wae-email）有实际测试

## Functional Requirements
- **FR-1**: 为所有核心模块添加单元测试覆盖
- **FR-2**: 建立集成测试基础设施
- **FR-3**: 添加基准测试支持
- **FR-4**: 配置测试覆盖率报告

## Non-Functional Requirements
- **NFR-1**: 测试套件能在 10 分钟内完整运行
- **NFR-2**: 核心模块的单元测试覆盖率至少达到 70%
- **NFR-3**: 基准测试能在合理时间内完成（< 5 分钟）
- **NFR-4**: 所有测试在 Windows、Linux 和 macOS 上都能通过

## Constraints
- **Technical**: 必须使用 Rust 内置的测试框架和 wae-testing 模块
- **Business**: 必须保持与现有 CI 配置的兼容性
- **Dependencies**: 依赖 wae-testing 模块提供的测试工具

## Assumptions
- 现有 wae-testing 模块的功能足够支持测试需求
- 所有模块的公共 API 已经稳定
- CI 环境有足够资源运行完整测试套件

## Acceptance Criteria

### AC-1: 单元测试覆盖核心模块
- **Given**: WAE 项目的所有后端模块
- **When**: 运行 `cargo test --workspace`
- **Then**: 所有核心模块都有单元测试，且测试通过
- **Verification**: `programmatic`
- **Notes**: 核心模块包括：wae-types, wae-config, wae-crypto, wae-resilience, wae-scheduler, wae-schema

### AC-2: 集成测试框架建立
- **Given**: WAE 项目 workspace 根目录
- **When**: 查看项目结构
- **Then**: 存在 tests/ 目录，包含集成测试基础设施
- **Verification**: `human-judgment`

### AC-3: 基准测试支持添加
- **Given**: wae-testing 模块
- **When**: 运行基准测试
- **Then**: 能够成功运行基准测试并获得性能指标
- **Verification**: `programmatic`

### AC-4: 测试覆盖率报告可生成
- **Given**: 完整的测试套件
- **When**: 运行覆盖率收集命令
- **Then**: 能生成 HTML 或 JSON 格式的覆盖率报告
- **Verification**: `programmatic`

### AC-5: CI 流程保持兼容
- **Given**: 现有的 CI 配置
- **When**: 提交代码触发 CI
- **Then**: CI 流程成功运行所有测试
- **Verification**: `programmatic`

## Open Questions
- [ ] 是否需要添加测试覆盖率门禁（最低覆盖率要求）？
- [ ] 基准测试是否需要在 CI 中定期运行？
