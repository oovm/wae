# 核心优势

## 1. 基于 axum 构建

WAE 基于 axum 框架构建，提供更便捷的构建器模式：

| 功能 | 说明 |
|-----|------|
| HTTP 服务 | 基于 axum Router 构建 |
| 构建器模式 | HttpsServerBuilder 链式配置 |
| 统一响应 | ApiResponse 统一 JSON 响应结构 |
| TLS 支持 | 内置 HTTPS 配置 |
| HTTP/2 | 支持 HTTP/2 协议 |
| CORS | 可选的跨域支持 |

### HttpsServerBuilder 用法

```rust
use wae_https::{HttpsServerBuilder, HttpsServerConfig, Http2Config};

let server = HttpsServerBuilder::new()
    .addr("0.0.0.0:3000".parse().unwrap())
    .service_name("my-service")
    .router(router)
    .tls("cert.pem", "key.pem")
    .http2_config(Http2Config::new().with_max_concurrent_streams(256))
    .build();

server.serve().await?;
```

### ApiResponse 统一响应结构

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

## 2. 微服务优先设计

### 服务实例信息

```rust
use wae_service::ServiceInstance;
use std::collections::HashMap;

let instance = ServiceInstance {
    id: "user-service-001".to_string(),
    name: "user-service".to_string(),
    addr: "192.168.1.100:8080".parse().unwrap(),
    metadata: HashMap::new(),
    tags: vec!["v1".to_string()],
    weight: 1,
    status: wae_service::ServiceStatus::Healthy,
    registered_at: chrono::Utc::now(),
    last_heartbeat: chrono::Utc::now(),
};
```

### 服务注册中心 Trait

```rust
use wae_service::{ServiceRegistry, ServiceInstance, ServiceStatus};



pub trait ServiceRegistry: Send + Sync {
    async fn register(&self, instance: &ServiceInstance) -> ServiceResult<()>;
    async fn deregister(&self, service_id: &str) -> ServiceResult<()>;
    async fn heartbeat(&self, service_id: &str) -> ServiceResult<()>;
    async fn update_status(&self, service_id: &str, status: ServiceStatus) -> ServiceResult<()>;
}
```

### 服务发现 Trait

```rust
use wae_service::{ServiceDiscovery, ServiceInstance, ServiceSubscription};



pub trait ServiceDiscovery: Send + Sync {
    async fn discover(&self, service_name: &str) -> ServiceResult<Vec<ServiceInstance>>;
    async fn discover_one(&self, service_name: &str) -> ServiceResult<Option<ServiceInstance>>;
    async fn subscribe(&self, service_name: &str) -> ServiceResult<Box<dyn ServiceSubscription>>;
}
```

### 负载均衡策略

```rust
use wae_service::{LoadBalancer, LoadBalanceStrategy};

let balancer = LoadBalancer::new(LoadBalanceStrategy::RoundRobin);

let instances = vec![instance1, instance2, instance3];
let selected = balancer.select(&instances);
```

支持的负载均衡策略：

| 策略 | 说明 |
|-----|------|
| RoundRobin | 轮询（默认） |
| Random | 随机选择 |
| WeightedRoundRobin | 加权轮询 |
| LeastConnections | 最少连接 |

当前仅提供内存实现，后续将支持 Consul、Nacos 等注册中心。

## 3. 配置管理

基于 figment 库实现，支持多层级配置合并：

**优先级**：环境变量 > 配置文件 > 默认值

### ConfigLoader 用法

```rust
use wae_config::{ConfigLoader, load_config};

#[derive(serde::Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
    database: DatabaseConfig,
}

let config: AppConfig = ConfigLoader::new()
    .with_toml("config.toml")
    .with_yaml("config.yaml")
    .with_env("APP_")
    .with_defaults(&default_config)
    .extract()?;
```

### TOML 配置文件

```toml
name = "my-service"
port = 8080

[database]
host = "localhost"
port = 5432
name = "mydb"
```

### YAML 配置文件

```yaml
name: my-service
port: 8080

database:
  host: localhost
  port: 5432
  name: mydb
```

### 环境变量加载

```rust
use wae_config::{ConfigLoader, from_env};

let config: AppConfig = ConfigLoader::new()
    .with_env("APP_")
    .extract()?;

let config: AppConfig = ConfigLoader::new()
    .with_env_separator("APP_", "__")
    .extract()?;

let config: AppConfig = from_env("APP_")?;
```

环境变量示例：

```bash
APP_NAME=my-service
APP_PORT=8080
APP_DATABASE__HOST=localhost
APP_DATABASE__PORT=5432
```

### 便捷函数

```rust
use wae_config::load_config;

let config: AppConfig = load_config("config.toml", "APP_")?;
```

## 4. 深度融合 tokio

### 原生异步 API

```rust
use wae_storage::{StorageProvider, StorageConfig, StorageResult};

use url::Url;


pub trait StorageProvider: Send + Sync {
    fn get_presigned_put_url(&self, key: &str, config: &StorageConfig) -> StorageResult<Url>;
    fn sign_url(&self, path: &str, config: &StorageConfig) -> StorageResult<Url>;
}
```

### 存储服务

```rust
use wae_storage::{StorageService, StorageConfig, StorageProviderType};

let config = StorageConfig {
    provider: StorageProviderType::Cos,
    secret_id: "your-secret-id".to_string(),
    secret_key: "your-secret-key".to_string(),
    bucket: "my-bucket".to_string(),
    region: "ap-guangzhou".to_string(),
    endpoint: None,
    cdn_url: None,
};

let url = StorageService::sign_url("/path/to/file", &config)?;
```

支持的存储类型：

| 类型 | 说明 |
|-----|------|
| Cos | 腾讯云对象存储 |
| Oss | 阿里云对象存储 |
| Local | 本地文件存储 |

## 5. 弹性容错

### 熔断器

```rust
use wae_resilience::{CircuitBreaker, CircuitBreakerConfig, CircuitState};

let circuit_breaker = CircuitBreaker::new(CircuitBreakerConfig::default());

let result = circuit_breaker.execute(|| async {
    some_external_service().await
}).await;
```

### 限流器

```rust
use wae_resilience::{TokenBucket, TokenBucketConfig, SlidingWindow, SlidingWindowConfig};

let rate_limiter = TokenBucket::new(TokenBucketConfig::default());
let sliding_window = SlidingWindow::new(SlidingWindowConfig::default());
```

### 重试

```rust
use wae_resilience::{RetryConfig, retry_async};

let result = retry_async(
    || async {
        some_external_service().await
    },
    RetryConfig::default(),
).await?;
```

### 超时

```rust
use wae_resilience::{TimeoutConfig, with_timeout};

let result = with_timeout(
    async { some_external_service().await },
    Duration::from_secs(5),
).await?;
```

### 弹性管道

```rust
use wae_resilience::{ResiliencePipeline, ResiliencePipelineBuilder};

let pipeline = ResiliencePipelineBuilder::new()
    .with_circuit_breaker(CircuitBreakerConfig::default())
    .with_retry(RetryConfig::default())
    .with_timeout(Duration::from_secs(10))
    .build();

let result = pipeline.execute(|| async {
    some_external_service().await
}).await;
```

## 6. 任务调度

### 定时任务

```rust
use wae_scheduler::{CronScheduler, CronSchedulerConfig, CronTask};

let scheduler = CronScheduler::new(CronSchedulerConfig::default());

scheduler.schedule("0 0 * * * *", my_task).await?;
scheduler.start().await?;
```

### 延迟任务

```rust
use wae_scheduler::{DelayedQueue, DelayedQueueConfig, DelayedTask};

let queue = DelayedQueue::new(DelayedQueueConfig::default());

queue.enqueue("task-1", Duration::from_secs(60)).await?;
```

### 间隔任务

```rust
use wae_scheduler::{IntervalScheduler, IntervalSchedulerConfig};

let scheduler = IntervalScheduler::new(IntervalSchedulerConfig::default());
scheduler.schedule("task-1", Duration::from_secs(60), my_task).await?;
scheduler.start().await?;
```

## 7. 认证服务

### JWT 认证

```rust
use wae_authentication::jwt::{JwtService, JwtConfig};

let jwt_service = JwtService::new(JwtConfig {
    secret: "your-secret-key".to_string(),
    issuer: "my-service".to_string(),
    audience: "my-api".to_string(),
    expires_in: 3600,
});

let token = jwt_service.generate(&claims)?;
let validation = jwt_service.validate(&token)?;
```

### OAuth2 认证

```rust
use wae_authentication::oauth2::{OAuth2Client, OAuth2Config, providers::{GitHubProvider, GoogleProvider}};

let github_client = OAuth2Client::new(GitHubProvider::new(config));
let google_client = OAuth2Client::new(GoogleProvider::new(config));

let auth_url = github_client.authorization_url();
let token = github_client.exchange_code(code).await?;
```

### TOTP 双因素认证

```rust
use wae_authentication::totp::{TotpService, TotpConfig};

let totp = TotpService::new(TotpConfig::default());
let secret = totp.generate_secret();
let code = totp.generate_code(&secret)?;
let valid = totp.verify(&secret, &code)?;
```

## 8. WebSocket 服务

### 服务端

```rust
use wae_websocket::{WebSocketServer, ServerConfig, ClientHandler, Connection, Message};

struct MyHandler;


impl ClientHandler for MyHandler {
    async fn on_connect(&self, conn: &Connection) -> WebSocketResult<()> {
        println!("Client connected: {}", conn.id);
        Ok(())
    }

    async fn on_message(&self, conn: &Connection, msg: Message) -> WebSocketResult<()> {
        println!("Received: {:?}", msg);
        Ok(())
    }

    async fn on_disconnect(&self, conn: &Connection) {
        println!("Client disconnected: {}", conn.id);
    }
}

let server = WebSocketServer::new(ServerConfig::default());
server.start(MyHandler).await?;
```

### 客户端

```rust
use wae_websocket::{WebSocketClient, ClientConfig, Message};

let client = WebSocketClient::new(ClientConfig::new("ws://localhost:8080"));

client.send_text("Hello").await?;

while let Some(msg) = client.receive().await {
    println!("Received: {:?}", msg);
}
```

## 9. 前端客户端

### HTTP 客户端

```typescript
import { createHttpClient } from "@wae/client";

const client = createHttpClient({
    baseUrl: "http://localhost:3000",
    timeout: 30000,
    maxRetries: 3,
});

const response = await client.get<UserData>("/users/1");

if (response.success) {
    console.log(response.data);
}
```

### 认证客户端

```typescript
import { createAuthClient } from "@wae/auth";

const auth = createAuthClient({
    baseUrl: "http://localhost:3000",
});

await auth.login({
    identifier: "user@example.com",
    password: "password",
});

console.log(auth.getState().isAuthenticated);
```

### WebSocket 客户端

```typescript
import { createWebSocketClient } from "@wae/websocket";

const ws = createWebSocketClient({
    url: "ws://localhost:8080",
    reconnectInterval: 5000,
});

ws.on("message", (data) => {
    console.log("Received:", data.message);
});

await ws.connect();
ws.sendText("Hello");
```

### 存储客户端

```typescript
import { createStorageClient } from "@wae/storage";

const storage = createStorageClient({
    baseUrl: "http://localhost:3000",
});

const result = await storage.uploadFile(file, "uploads/test.jpg", (progress) => {
    console.log(`Progress: ${progress.percentage}%`);
});

console.log("Access URL:", result.accessUrl);
```

## 10. AI 友好设计

### 清晰的 Trait 抽象

```rust



pub trait ServiceRegistry: Send + Sync {
    async fn register(&self, instance: &ServiceInstance) -> ServiceResult<()>;
    async fn deregister(&self, service_id: &str) -> ServiceResult<()>;
    async fn heartbeat(&self, service_id: &str) -> ServiceResult<()>;
    async fn update_status(&self, service_id: &str, status: ServiceStatus) -> ServiceResult<()>;
}
```

AI 可以通过 trait 定义清晰理解：
- 输入参数类型
- 输出结果类型
- 异步行为
- 错误处理方式

### 统一的错误处理

```rust
use wae_types::CloudError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CloudError {
    Authentication(String),
    InvalidParams(String),
    NotFound(String),
    PermissionDenied(String),
    RateLimit(String),
    Internal(String),
    Network(String),
    Unknown(String),
}
```

## 11. 纯 Rust 优先

WAE 坚持纯 Rust 实现，带来以下优势：

| 特性 | 说明 |
|-----|------|
| 零 FFI 绑定 | 避免 C 库依赖，编译简单 |
| 跨平台支持 | Windows/Linux/macOS 无差异 |
| 安全性 | Rust 的内存安全保证 |
| 性能 | 无 FFI 开销 |
| 可审计性 | 所有代码可见可审计 |
| tokio 原生 | 与 tokio 生态无缝集成 |
