# wae-https 核心功能 - 实现计划

## [/] 任务 1: 路由系统核心实现
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - 使用 matchit 实现路由匹配引擎
  - 实现 Router 结构体，存储路由和处理函数
  - 实现 RouterBuilder 的路由添加方法（get, post, put, delete 等）
  - 支持路由合并和嵌套
- **Acceptance Criteria Addressed**: AC-1, AC-9, AC-10
- **Test Requirements**:
  - `programmatic` TR-1.1: 能够添加各种 HTTP 方法的路由并匹配成功
  - `programmatic` TR-1.2: 路由合并和嵌套功能正常
  - `human-judgement` TR-1.3: 所有公共 API 都有文档注释
- **Notes**: 保持现有 API 接口不变

## [ ] 任务 2: 提取器系统实现
- **Priority**: P0
- **Depends On**: 任务 1
- **Description**:
  - 实现 Path 参数提取器
  - 实现 Query 参数提取器
  - 实现 Json 提取器
  - 实现 Form 提取器
  - 实现处理函数和提取器的绑定机制
- **Acceptance Criteria Addressed**: AC-2, AC-3, AC-4, AC-5, AC-10
- **Test Requirements**:
  - `programmatic` TR-2.1: 路径参数能正确提取
  - `programmatic` TR-2.2: 查询参数能正确提取和反序列化
  - `programmatic` TR-2.3: JSON 体能正确提取和反序列化
  - `programmatic` TR-2.4: Form 体能正确提取和反序列化
  - `human-judgement` TR-2.5: 提取器实现有完整文档注释

## [ ] 任务 3: 服务器启动实现
- **Priority**: P0
- **Depends On**: 任务 1, 任务 2
- **Description**:
  - 使用 hyper-util 实现 serve_plain
  - 实现 serve_tls（使用 tokio-rustls）
  - 将 Router 转换为 hyper 服务
  - 实现请求分发到处理函数
- **Acceptance Criteria Addressed**: AC-6, AC-8, AC-9, AC-10
- **Test Requirements**:
  - `programmatic` TR-3.1: HTTP 服务器能正常启动和处理请求
  - `programmatic` TR-3.2: HTTPS 服务器能正常启动和处理请求（使用自签名证书测试）
  - `programmatic` TR-3.3: 请求能正确分发到处理函数
  - `human-judgement` TR-3.4: 服务器实现有完整文档注释

## [ ] 任务 4: 中间件集成
- **Priority**: P1
- **Depends On**: 任务 3
- **Description**:
  - 实现中间件层集成
  - 支持 tower 和 tower-http 的中间件
  - 确保现有中间件（cors, compression 等）能正常使用
- **Acceptance Criteria Addressed**: AC-6, AC-9, AC-10
- **Test Requirements**:
  - `programmatic` TR-4.1: 中间件能正确应用到路由
  - `programmatic` TR-4.2: 现有中间件能正常工作
  - `human-judgement` TR-4.3: 中间件集成有完整文档注释

## [ ] 任务 5: 静态文件服务实现
- **Priority**: P1
- **Depends On**: 任务 4
- **Description**:
  - 使用 tower-http 的 ServeDir 或类似实现
  - 实现 static_files_router 函数
  - 支持正确的 MIME 类型和缓存头
- **Acceptance Criteria Addressed**: AC-7, AC-9, AC-10
- **Test Requirements**:
  - `programmatic` TR-5.1: 静态文件能正确提供
  - `programmatic` TR-5.2: 正确的 MIME 类型被设置
  - `human-judgement` TR-5.3: 静态文件服务有完整文档注释

## [ ] 任务 6: 测试和文档完善
- **Priority**: P2
- **Depends On**: 任务 5
- **Description**:
  - 完善现有测试用例
  - 编写新的测试覆盖所有新功能
  - 确保所有文档注释完整
- **Acceptance Criteria Addressed**: AC-10
- **Test Requirements**:
  - `programmatic` TR-6.1: 所有测试通过
  - `human-judgement` TR-6.2: 所有公共 API 都有文档注释
