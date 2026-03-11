# WAE Crypto 增强 - Product Requirement Document

## Overview
- **Summary**: 增强和完善 wae-crypto 模块，实现完整的加密/哈希功能抽象层，并确保整个项目的加密操作都通过 wae-crypto API 进行。
- **Purpose**: 解决 wae-crypto 现有功能不完整（如 argon2 尚未实现），以及项目中其他模块直接使用底层加密库的问题，提供统一、安全、易于维护的加密抽象层。
- **Target Users**: WAE 框架的开发者和使用者。

## Goals
- 完善 wae-crypto 的现有功能，实现 argon2 密码哈希算法
- 确保所有 public 的结构体、枚举、方法、字段都有文档注释
- 确保整个项目的加密操作都通过 wae-crypto API 进行，不直接使用底层库
- 保持并增强 wae-crypto 已有的 trait 抽象和封装

## Non-Goals (Out of Scope)
- 自主实现轻量级的哈希算法（使用现有成熟库）
- 重新设计 wae-crypto 的架构（保持现有架构）
- 添加新的加密算法（除非是完善现有功能所需）

## Background & Context
- wae-crypto 已经有很好的抽象层，包括 PasswordHasher、HashAlgorithm 等 trait 和枚举
- 当前依赖：bcrypt (0.19.0)、argon2 (0.5.3)、hmac (0.12.1)、sha1/sha2 (0.10.6/0.10.9)、base64 (0.22.1)
- 已有 base64 封装
- 但是 argon2 密码哈希功能尚未实现（目前返回 InvalidAlgorithm 错误）
- wae-authentication 等模块直接使用 bcrypt 等底层库，而不是通过 wae-crypto

## Functional Requirements
- **FR-1**: 实现 wae-crypto 中 argon2 密码哈希和验证功能
- **FR-2**: 增强和完善 wae-crypto 的文档注释
- **FR-3**: 更新 wae-authentication 模块，使其使用 wae-crypto 的密码哈希功能，而不是直接使用 bcrypt
- **FR-4**: 检查并确保项目中其他模块的加密操作都通过 wae-crypto API 进行

## Non-Functional Requirements
- **NFR-1**: 所有功能必须经过测试验证，确保正确性和安全性
- **NFR-2**: 保持向后兼容性，不破坏现有代码的功能
- **NFR-3**: 代码必须符合项目的编码规范和风格

## Constraints
- **Technical**: Rust 语言，使用现有依赖库（bcrypt、argon2、hmac、sha1、sha2、base64）
- **Business**: 中高优先级，需要尽快完成
- **Dependencies**: 依赖 wae-types、zeroize 等现有库

## Assumptions
- 现有 wae-crypto 的架构设计是合理的，不需要大的改动
- 项目的其他模块愿意迁移到使用 wae-crypto API
- 底层依赖库的版本不需要更新

## Acceptance Criteria

### AC-1: Argon2 密码哈希功能实现
- **Given**: wae-crypto 模块已存在，且配置为使用 Argon2 算法
- **When**: 调用 PasswordHasher.hash_password() 方法
- **Then**: 应该成功生成 Argon2 格式的密码哈希
- **Verification**: `programmatic`
- **Notes**: 需要测试不同的密码和配置

### AC-2: Argon2 密码验证功能实现
- **Given**: 有一个使用 Argon2 生成的密码哈希
- **When**: 调用 PasswordHasher.verify_password() 方法验证该哈希
- **Then**: 应该正确验证密码的正确性
- **Verification**: `programmatic`

### AC-3: wae-crypto 文档注释完善
- **Given**: wae-crypto 模块的所有源代码
- **When**: 检查所有 public 的结构体、枚举、方法、字段
- **Then**: 每个都应该有适当的文档注释，且不使用后置注释
- **Verification**: `human-judgment`

### AC-4: wae-authentication 使用 wae-crypto
- **Given**: wae-authentication 模块的 PasswordHasherService
- **When**: 调用其 hash_password() 和 verify_password() 方法
- **Then**: 应该通过 wae-crypto 的 API 来实现，而不是直接使用 bcrypt
- **Verification**: `programmatic`

### AC-5: 项目加密操作统一使用 wae-crypto
- **Given**: 整个项目的源代码
- **When**: 检查所有模块的加密操作（哈希、HMAC、base64 等）
- **Then**: 应该都通过 wae-crypto 的 API 进行，不直接使用底层库
- **Verification**: `human-judgment`

## Open Questions
- [ ] 是否需要添加更多的密码哈希算法配置选项？
- [ ] 除了 wae-authentication 外，其他模块是否需要更新以使用 wae-crypto？
