# wae-effect 依赖注入/效果系统增强 - Verification Checklist

- [ ] 类型安全的依赖注册和获取功能正常工作
- [ ] Singleton 作用域依赖始终返回同一实例
- [ ] Request-scoped 作用域依赖在不同请求中返回不同实例
- [ ] use_xxx!() 宏能正确展开并获取依赖
- [ ] 所有现有 API 保持向后兼容
- [ ] 所有新功能都有完整的文档注释
- [ ] 所有测试通过
