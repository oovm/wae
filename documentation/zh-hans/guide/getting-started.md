# 快速开始

本节将帮助你在 5 分钟内创建一个基于 WAE 的 HTTP 服务。

## 安装

### 后端 (Rust)

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
wae = { path = "wae" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

### 前端 (TypeScript)

```bash
npm install @wae/core @wae/client
```

## 基础 HTTP 服务

创建一个使用 WAE 的 HTTP 服务：

```rust
use axum::{Router, routing::{get, post}, Json};
use wae_https::{HttpsServerBuilder, ApiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    username: String,
    email: Option<String>,
}

async fn health() -> &'static str {
    "OK"
}

async fn hello() -> ApiResponse<HelloResponse> {
    ApiResponse::success(HelloResponse {
        message: "Hello, WAE!".to_string(),
    })
}

async fn create_user(
    Json(req): Json<CreateUserRequest>,
) -> ApiResponse<HelloResponse> {
    ApiResponse::success(HelloResponse {
        message: format!("Created user: {}", req.username),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
        .route("/users", post(create_user));

    let server = HttpsServerBuilder::new()
        .addr("0.0.0.0:3000".parse()?)
        .service_name("my-service")
        .router(router)
        .build();

    server.serve().await?;

    Ok(())
}
```

## 运行

```bash
cargo run
```

访问 `http://localhost:3000/health` 查看健康检查端点。

## HTTPS 配置

启用 HTTPS 只需添加 TLS 配置：

```rust
let server = HttpsServerBuilder::new()
    .addr("0.0.0.0:443".parse()?)
    .service_name("my-secure-service")
    .router(router)
    .tls("cert.pem", "key.pem")
    .build();
```

## HTTP/2 配置

WAE 支持 HTTP/2 协议：

```rust
use wae_https::{HttpsServerBuilder, Http2Config};

let http2_config = Http2Config::new()
    .with_max_concurrent_streams(256)
    .with_enable_push(false);

let server = HttpsServerBuilder::new()
    .addr("0.0.0.0:443".parse()?)
    .service_name("my-http2-service")
    .router(router)
    .tls("cert.pem", "key.pem")
    .http2_config(http2_config)
    .build();
```

## 统一响应结构

WAE 提供统一的 JSON 响应结构 `ApiResponse`：

```rust
use wae_https::ApiResponse;

#[derive(serde::Serialize)]
struct UserData {
    id: u64,
    name: String,
}

async fn get_user() -> ApiResponse<UserData> {
    ApiResponse::success(UserData {
        id: 1,
        name: "Alice".to_string(),
    })
}

async fn create_user() -> ApiResponse<()> {
    ApiResponse::error("INVALID_PARAMS", "用户名已存在")
}
```

响应格式：

```json
{
    "success": true,
    "data": { "id": 1, "name": "Alice" },
    "error": null,
    "trace_id": null
}
```

## 前端客户端

使用 TypeScript 客户端调用后端 API：

```typescript
import { createHttpClient } from "@wae/client";

const client = createHttpClient({
    baseUrl: "http://localhost:3000",
    timeout: 30000,
});

const response = await client.get<UserData>("/hello");

if (response.success) {
    console.log(response.data.message);
}
```

## 配置管理

使用 wae-config 加载配置：

```rust
use wae_config::{ConfigLoader, load_config};
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
    database: DatabaseConfig,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    name: String,
}

let config: AppConfig = ConfigLoader::new()
    .with_toml("config.toml")
    .with_env("APP_")
    .extract()?;
```

TOML 配置文件：

```toml
name = "my-service"
port = 8080

[database]
host = "localhost"
port = 5432
name = "mydb"
```

环境变量：

```bash
APP_NAME=my-service
APP_PORT=8080
APP_DATABASE__HOST=localhost
APP_DATABASE__PORT=5432
```

## 下一步

- [核心优势](/guide/advantages) - 了解 WAE 的设计理念
- [架构设计](/architecture/overview) - 深入了解 WAE 的架构
- [模块文档](/modules/ai) - 了解各个模块的功能
