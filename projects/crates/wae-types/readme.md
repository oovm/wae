# schema（crate `wae-types`）

Rust 侧通信 **IDL / 类型真理源**。目标：codegen 生成 `@wae/types`（及后续绑定）。**不是** npm 包。

## 谁该读这份说明

- 要改跨语言消息形状的人
- 要实现 `wae generate types` 的人
- 需要在 Rust host 里引用同一套 `ClientMessage` / `HostMessage` 的人

## 定义与生成（意图）

```text
schema（Rust）
  → wae generate types（CLI，0.0.0 骨架）
  → @wae/types（TypeScript）
```

当前仓库里 `@wae/types` 为手写对齐稿；改 schema 后 **不会**自动更新 npm 类型，直到 codegen 接线。

## 变更影响

| 变更 | 影响 |
|------|------|
| 新增可选字段 | 旧客户端可能忽略；需约定默认值 |
| 删除 / 改名字段 | 破坏 TS 与 Rust 两侧 |
| 改 `ErrorCode` | 错误处理分支全线受影响 |

运行时校验：TS 侧 `protocol` 目前只做 `JSON.parse`；严格校验若需要，应落在 codegen 产物或独立校验层。

## 开发

```bash
cargo check -p wae-types
```

与 npm 发布无关；应用用户通常只依赖 `@wae/types` / `@wae/protocol`。
