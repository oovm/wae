# wae-schema

Schema 模块 - 提供数据结构定义和验证。

## 主要功能

- **Schema 定义**: 类型安全的数据结构
- **数据验证**: 自动验证数据有效性
- **序列化支持**: JSON/消息序列化
- **文档生成**: 自动生成 API 文档

## 技术栈

- **验证**: validator
- **序列化**: serde
- **文档**: utoipa (可选)

## 使用示例

```rust
use wae_schema::{Schema, Validate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Schema, Serialize, Deserialize)]
pub struct UserSchema {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(range(min = 0, max = 150))]
    pub age: u32,
}

let user = UserSchema {
    username: "张三".to_string(),
    email: "zhangsan@example.com".to_string(),
    age: 25,
};

user.validate()?;
```

## 嵌套 Schema

```rust
#[derive(Debug, Clone, Schema)]
pub struct OrderSchema {
    pub id: String,
    pub user: UserSchema,
    #[validate(length(min = 1))]
    pub items: Vec<OrderItemSchema>,
}
```
