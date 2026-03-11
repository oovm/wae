# wae-database 生态完善 - The Implementation Plan (Decomposed and Prioritized Task List)

## [ ] Task 1: 数据库连接提取器实现
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 在 wae-database 中添加与 HTTP 框架集成的提取器
  - 实现 DatabaseConnection 提取器，支持从请求扩展中获取连接
  - 支持多种数据库后端（PostgreSQL、MySQL、Turso）
- **Acceptance Criteria Addressed**: [AC-1, AC-5]
- **Test Requirements**:
  - `programmatic` TR-1.1: 可以通过提取器获取数据库连接
  - `programmatic` TR-1.2: 支持所有三种数据库后端
  - `human-judgement` TR-1.3: 所有公共 API 有完整文档注释
- **Notes**: 应遵循现有的 wae-https extract 模块的设计模式

## [ ] Task 2: 事务中间件实现
- **Priority**: P0
- **Depends On**: [Task 1]
- **Description**:
  - 实现事务中间件，支持请求级别的事务管理
  - 自动在请求开始时开启事务，请求成功时提交，失败时回滚
  - 提供可配置的事务选项
- **Acceptance Criteria Addressed**: [AC-2, AC-5]
- **Test Requirements**:
  - `programmatic` TR-2.1: 请求成功时事务自动提交
  - `programmatic` TR-2.2: 请求失败时事务自动回滚
  - `human-judgement` TR-2.3: 所有公共 API 有完整文档注释
- **Notes**: 应遵循现有的 wae-https middleware 模块的设计模式

## [ ] Task 3: 完善 auto-migrate 功能
- **Priority**: P0
- **Depends On**: None
- **Description**:
  - 完善现有的 auto-migrate 框架实现
  - 支持表创建、列添加/修改/删除
  - 支持索引创建和删除
  - 支持外键约束管理
- **Acceptance Criteria Addressed**: [AC-3, AC-5]
- **Test Requirements**:
  - `programmatic` TR-3.1: 可以自动创建新表
  - `programmatic` TR-3.2: 可以添加、修改、删除列
  - `programmatic` TR-3.3: 可以管理索引
  - `human-judgement` TR-3.4: 所有公共 API 有完整文档注释
- **Notes**: 应先查看现有 auto_migrate 模块的实现

## [ ] Task 4: 连接池深度优化
- **Priority**: P1
- **Depends On**: None
- **Description**:
  - 优化连接池配置选项
  - 添加连接健康检查
  - 优化连接回收策略
  - 添加连接池监控指标
- **Acceptance Criteria Addressed**: [AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-4.1: 连接池参数可配置
  - `programmatic` TR-4.2: 连接健康检查正常工作
  - `human-judgement` TR-4.3: 所有公共 API 有完整文档注释
- **Notes**: 针对 PostgreSQL 使用 deadpool-postgres，其他后端相应优化

## [ ] Task 5: 集成测试和文档完善
- **Priority**: P1
- **Depends On**: [Task 1, Task 2, Task 3, Task 4]
- **Description**:
  - 编写集成测试，测试所有新功能
  - 确保所有文档注释完整
  - 添加使用示例
- **Acceptance Criteria Addressed**: [AC-1, AC-2, AC-3, AC-4, AC-5]
- **Test Requirements**:
  - `programmatic` TR-5.1: 所有集成测试通过
  - `human-judgement` TR-5.2: 文档注释完整且正确
