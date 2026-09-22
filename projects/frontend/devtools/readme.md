# frontend/devtools

开发期辅助占位目录，**不是**应用运行时核心，也**不是**公开 npm 产品面。

## 状态

- 不保证可运行、不参与 `0.0.0` 发布。
- **不要**依赖已删除的 `@wae/adapter-vanilla` 或 `@wae/ui`。
- 纯 TS/DOM 调试请直接用 [`@wae/client`](../runtime/readme.md)。

## 新路径

| 旧想法 | 现在 |
|--------|------|
| vanilla adapter | 不存在；用 `@wae/client` |
| 自有 UI 框架 | 不存在；用你的 Vue/React/Svelte/Solid |
