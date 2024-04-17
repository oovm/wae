# effect-http-server

代数效应 HTTP 服务器示例 - 演示如何在 Axum 中使用 WAE Effect。

## 示例说明

本示例展示如何将 `wae-effect` 与 Axum Web 框架集成，实现请求级别的依赖注入。

## 主要功能演示

- 在 Axum 中使用代数效应
- 通过 `Effectful` 提取器获取依赖
- 实现类型安全的请求处理

## 运行示例

```bash
cd examples/effect-http-server
cargo run
```

服务器将在 `http://localhost:3000` 启动。

## API 端点

| 端点 | 方法 | 说明 |
|------|------|------|
| `/health` | GET | 健康检查 |
| `/api/user` | GET | 获取当前用户 |

## 代码示例

```rust
use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use wae_effect::{AlgebraicEffect, EffectError, Effectful};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
struct AppConfig {
    app_name: String,
    version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
}

async fn health(ae: Effectful) -> Result<Json<HealthResponse>, EffectError> {
    let config = ae.use_config::<AppConfig>().await?;
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        app_name: config.app_name,
        version: config.version,
    }))
}

async fn get_user(ae: Effectful) -> Result<Json<UserResponse>, EffectError> {
    let user = ae.use_auth::<User>().await?;
    Ok(Json(UserResponse { user }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deps = AlgebraicEffect::new()
        .with_config(AppConfig {
            app_name: "WAE Effect HTTP Server".to_string(),
            version: "0.1.0".to_string(),
        })
        .with_auth(User {
            id: "user-001".to_string(),
            username: "demo_user".to_string(),
            email: "demo@example.com".to_string(),
        })
        .build();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/user", get(get_user))
        .with_state(deps);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

## 测试 API

```bash
curl http://localhost:3000/health
# {"status":"ok","app_name":"WAE Effect HTTP Server","version":"0.1.0"}

curl http://localhost:3000/api/user
# {"user":{"id":"user-001","username":"demo_user","email":"demo@example.com"}}
```

## 依赖

- `wae` - WAE 框架入口
- `wae-effect` - 代数效应模块
- `wae-database` - 数据库模块 (可选)
- `axum` - Web 框架
