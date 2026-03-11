# wae-effect 依赖注入/效果系统增强 - The Implementation Plan

## [x] Task 1: 增强 Dependencies 以支持类型安全（基于 TypeId）
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 在 Dependencies 结构体中添加基于 TypeId 的存储
  - 添加 register_type 方法用于按类型注册依赖
  - 添加 get_type 方法用于按类型获取依赖
  - 保持现有字符串键 API 不变（向后兼容）
- **Acceptance Criteria Addressed**: [AC-1, AC-5]
- **Test Requirements**:
  - `programmatic` TR-1.1: 可以按类型注册和获取依赖
  - `programmatic` TR-1.2: 类型不匹配时编译错误
  - `programmatic` TR-1.3: 现有字符串键 API 继续正常工作
- **Notes**: 使用 std::any::TypeId 和 std::any::Any

## [x] Task 2: 实现作用域系统（Singleton, Request-scoped）
- **Priority**: P0
- **Depends On**: [Task 1]
- **Description**: 
  - 定义 Scope 枚举（Singleton, RequestScoped）
  - 为 Dependencies 中的依赖添加作用域元数据
  - 实现 Effectful 对 Request-scoped 依赖的支持
- **Acceptance Criteria Addressed**: [AC-2, AC-3]
- **Test Requirements**:
  - `programmatic` TR-2.1: Singleton 依赖在多次获取时返回同一实例
  - `programmatic` TR-2.2: Request-scoped 依赖在不同请求中返回不同实例
- **Notes**: Request-scoped 依赖可以在 Effectful 中独立存储

## [ ] Task 3: 为 AlgebraicEffect 和 Effectful 添加类型安全的便捷方法
- **Priority**: P1
- **Depends On**: [Task 2]
- **Description**: 
  - AlgebraicEffect 添加 with_type 方法
  - Effectful 添加 use_type 方法
  - 实现 with_config, use_config, with_auth, use_auth 等常用便捷方法
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3]
- **Test Requirements**:
  - `programmatic` TR-3.1: with_type 和 use_type 方法正常工作
  - `programmatic` TR-3.2: 便捷方法（with_config, use_config 等）正常工作

## [ ] Task 4: 在 wae-macros 中实现 use_xxx!() 宏
- **Priority**: P1
- **Depends On**: [Task 3]
- **Description**: 
  - 实现 use_effect! 宏
  - 支持多种语法（获取任意类型、Arc 类型、config、auth、database 等）
  - 添加完整的文档注释
- **Acceptance Criteria Addressed**: [AC-4]
- **Test Requirements**:
  - `programmatic` TR-4.1: use_effect! 宏能正确展开并获取依赖
  - `programmatic` TR-4.2: 宏的各种语法变体都能正常工作

## [ ] Task 5: 编写完整的测试
- **Priority**: P1
- **Depends On**: [Task 1, Task 2, Task 3, Task 4]
- **Description**: 
  - 为类型安全 API 编写测试
  - 为作用域系统编写测试
  - 为宏编写测试
  - 确保所有现有测试仍然通过
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-5.1: 所有新功能测试通过
  - `programmatic` TR-5.2: 所有现有测试通过

## [ ] Task 6: 更新文档和示例
- **Priority**: P2
- **Depends On**: [Task 5]
- **Description**: 
  - 更新 effect.md 文档以反映新功能
  - 更新或添加新的示例代码
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-4]
- **Test Requirements**:
  - `human-judgement` TR-6.1: 文档完整且准确
