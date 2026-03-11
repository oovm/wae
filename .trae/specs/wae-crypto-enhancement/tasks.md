# WAE Crypto 增强 - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: 实现 Argon2 密码哈希和验证功能
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 在 wae-crypto 的 password.rs 中实现 hash_argon2() 和 verify_argon2() 方法
  - 可能需要扩展 PasswordHasherConfig 以支持 Argon2 配置选项
  - 使用 argon2 库的标准 API 实现
- **Acceptance Criteria Addressed**: [AC-1, AC-2]
- **Test Requirements**:
  - `programmatic` TR-1.1: 测试使用 Argon2 算法哈希密码成功
  - `programmatic` TR-1.2: 测试使用 Argon2 算法验证密码成功
  - `programmatic` TR-1.3: 测试错误密码验证失败
- **Notes**: 确保与 bcrypt 的实现保持一致的 API 和错误处理

## [ ] Task 2: 增强和完善 wae-crypto 的文档注释
- **Priority**: P1
- **Depends On**: None
- **Description**:
  - 检查 wae-crypto 所有文件的 public API
  - 为缺少文档注释的结构体、枚举、方法、字段添加文档注释
  - 确保不使用后置注释
- **Acceptance Criteria Addressed**: [AC-3]
- **Test Requirements**:
  - `human-judgement` TR-2.1: 检查所有 public 项都有适当的文档注释
  - `human-judgement` TR-2.2: 检查没有使用后置注释
- **Notes**: 参考现有代码的文档注释风格

## [ ] Task 3: 更新 wae-authentication 使用 wae-crypto
- **Priority**: P1
- **Depends On**: [Task 1]
- **Description**:
  - 修改 wae-authentication 的 PasswordHasherService
  - 移除直接使用 bcrypt 的代码
  - 改用 wae-crypto 的 PasswordHasher API
  - 更新相关的配置和错误处理
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `programmatic` TR-3.1: 测试 wae-authentication 的密码哈希功能正常
  - `programmatic` TR-3.2: 测试 wae-authentication 的密码验证功能正常
  - `programmatic` TR-3.3: 检查 wae-authentication 不再直接依赖 bcrypt
- **Notes**: 保持向后兼容性，确保现有功能不受影响

## [ ] Task 4: 检查并统一项目其他模块的加密操作
- **Priority**: P2
- **Depends On**: [Task 1, Task 2, Task 3]
- **Description**:
  - 搜索项目中直接使用 bcrypt、argon2、hmac、sha1、sha2、base64 的代码
  - 评估哪些可以迁移到使用 wae-crypto API
  - 逐步迁移合适的模块
- **Acceptance Criteria Addressed**: [AC-5]
- **Test Requirements**:
  - `human-judgement` TR-4.1: 检查项目中适合迁移的模块已使用 wae-crypto
  - `programmatic` TR-4.2: 确保迁移后的功能正常工作
- **Notes**: 优先考虑安全相关的模块，避免破坏现有功能
