# WAE 数据库替换 - Turso 改为 Limbo - 验证清单

- [ ] 检查 Cargo.toml 中是否已移除 Turso 依赖
- [ ] 检查 Cargo.toml 中是否已添加 Limbo 依赖
- [ ] 检查是否存在 limbo.rs 文件
- [ ] 检查 limbo.rs 文件是否实现了所有必要的功能
- [ ] 检查 mod.rs 文件是否正确导出 Limbo 相关类型
- [ ] 检查 config.rs 文件是否正确定义 Limbo 配置
- [ ] 检查 trait_impl.rs 文件是否正确更新 DatabaseBackend 枚举
- [ ] 检查 row.rs 文件是否正确实现 Limbo 相关的行处理
- [ ] 检查 statement.rs 文件是否正确实现 Limbo 相关的语句处理
- [ ] 检查 types.rs 文件是否正确实现 Limbo 相关的类型转换
- [ ] 检查是否已删除 turso.rs 文件
- [ ] 运行 wae-database 模块的测试，确保所有测试通过
- [ ] 运行整个项目的测试，确保所有测试通过
- [ ] 检查代码注释和文档是否已更新