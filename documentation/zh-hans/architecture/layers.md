# 模块层次

WAE 采用清晰的模块层次结构，从基础类型到业务应用逐层构建。

## 层次结构

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  业务应用层：用户服务、订单服务、内容服务等                      │
├─────────────────────────────────────────────────────────────┤
│                     Service Layer                            │
│  服务层：wae-ai, wae-storage, wae-email, wae-authentication  │
├─────────────────────────────────────────────────────────────┤
│                     Infrastructure Layer                     │
│  基础设施层：wae-https, wae-websocket, wae-resilience,       │
│             wae-scheduler, wae-service, wae-config           │
├─────────────────────────────────────────────────────────────┤
│                     Foundation Layer                         │
│  基础层：wae-types, wae-macros                               │
└─────────────────────────────────────────────────────────────┘
```

## 基础层 (Foundation Layer)

提供核心类型定义和工具宏：

### wae-types

```rust
/// 统一错误类型
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

/// 统一结果类型
pub type CloudResult<T> = Result<T, CloudError>;

/// 计费维度
pub struct BillingDimensions {
    pub input_text: u64,
    pub output_text: u64,
    pub input_pixels: u64,
    pub output_pixels: u64,
}

/// 动态值类型
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
```

### wae-macros

提供过程宏支持：

```rust
/// 查询构建宏
#[wae::query]
fn find_user(id: u64) -> User {
    "SELECT * FROM users WHERE id = $1"
}
```

## 基础设施层 (Infrastructure Layer)

提供通用的基础设施能力：

### wae-https

基于 axum 的 HTTP 服务：

```rust
use wae_https::{HttpsServerBuilder, ApiResponse, Http2Config};

let server = HttpsServerBuilder::new()
    .addr("0.0.0.0:3000".parse()?)
    .service_name("my-service")
    .router(router)
    .tls("cert.pem", "key.pem")
    .http2_config(Http2Config::new())
    .build();

server.serve().await?;
```

### wae-websocket

WebSocket 双向通信：

```rust
use wae_websocket::{WebSocketServer, ServerConfig, ClientHandler};

let server = WebSocketServer::new(ServerConfig::default());
server.start(MyHandler).await?;
```

### wae-resilience

弹性容错能力：

```rust
use wae_resilience::{CircuitBreaker, RateLimiter, RetryConfig};

let pipeline = ResiliencePipelineBuilder::new()
    .with_circuit_breaker(CircuitBreakerConfig::default())
    .with_retry(RetryConfig::default())
    .with_timeout(Duration::from_secs(10))
    .build();
```

### wae-scheduler

任务调度：

```rust
use wae_scheduler::{CronScheduler, DelayedQueue, IntervalScheduler};

let scheduler = CronScheduler::new(CronSchedulerConfig::default());
scheduler.schedule("0 0 * * * *", my_task).await?;
```

### wae-service

服务发现与注册：

```rust
use wae_service::{ServiceRegistry, ServiceDiscovery, LoadBalancer};

let balancer = LoadBalancer::new(LoadBalanceStrategy::RoundRobin);
let instance = balancer.select(&instances);
```

### wae-config

配置管理：

```rust
use wae_config::ConfigLoader;

let config: AppConfig = ConfigLoader::new()
    .with_toml("config.toml")
    .with_env("APP_")
    .extract()?;
```

## 服务层 (Service Layer)

提供业务无关的服务抽象：

### wae-authentication

认证服务：

```rust
use wae_authentication::{
    jwt::JwtService,
    oauth2::{OAuth2Client, providers::GitHubProvider},
    totp::TotpService,
    AuthService,
};

let auth_service = MemoryAuthService::new(AuthConfig::default());
let token = auth_service.login(&credentials).await?;
```

### wae-storage

对象存储：

```rust
use wae_storage::{StorageService, StorageConfig, StorageProviderType};

let config = StorageConfig {
    provider: StorageProviderType::Cos,
    secret_id: "id".to_string(),
    secret_key: "key".to_string(),
    bucket: "bucket".to_string(),
    region: "region".to_string(),
    endpoint: None,
    cdn_url: None,
};

let url = StorageService::sign_url("/path", &config)?;
```

### wae-ai

AI 服务：

```rust
use wae_ai::{HunyuanProvider, ChatCapability, TextToImageCapability};

let provider = HunyuanProvider::new(&config);
let response = provider.chat(&params, &ai_config).await?;
```

### wae-email

邮件服务：

```rust
use wae_email::{SmtpEmailProvider, EmailProvider};

let email = SmtpEmailProvider::new(smtp_config);
email.send(&email_message).await?;
```

## 应用层 (Application Layer)

业务应用实现：

```rust
use wae::prelude::*;

struct UserService {
    auth: Arc<dyn AuthService>,
    storage: Arc<StorageService>,
    db: Database,
}

impl UserService {
    async fn register(&self, req: CreateUserRequest) -> Result<UserInfo, CloudError> {
        let user = self.auth.create_user(&req).await?;
        self.storage.upload_avatar(&user.id, &req.avatar).await?;
        Ok(user)
    }
}
```

## 前端层次

前端同样采用分层结构：

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  React/Vue 组件、页面、业务逻辑                               │
├─────────────────────────────────────────────────────────────┤
│                     Client Layer                             │
│  @wae/auth, @wae/websocket, @wae/storage                     │
├─────────────────────────────────────────────────────────────┤
│                     Core Layer                               │
│  @wae/core (类型定义), @wae/client (HTTP 客户端)             │
└─────────────────────────────────────────────────────────────┘
```

### @wae/core

核心类型定义：

```typescript
export interface CloudError {
    type: CloudErrorType;
    message: string;
}

export interface ApiResponse<T> {
    success: boolean;
    data: T | null;
    error: ApiErrorBody | null;
    traceId: string | null;
}
```

### @wae/client

HTTP 客户端：

```typescript
export class HttpClient {
    async get<T>(url: string): Promise<ApiResponse<T>>;
    async post<T>(url: string, body?: unknown): Promise<ApiResponse<T>>;
    async put<T>(url: string, body?: unknown): Promise<ApiResponse<T>>;
    async delete<T>(url: string): Promise<ApiResponse<T>>;
}
```

### @wae/auth

认证客户端：

```typescript
export class AuthClient {
    async login(credentials: Credentials): Promise<AuthToken>;
    async logout(): Promise<void>;
    async refreshToken(): Promise<AuthToken>;
    getState(): AuthState;
    subscribe(listener: AuthStateListener): () => void;
}
```

## 层次间通信

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ Application │────▶│   Service   │────▶│Infrastructure│
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │                    │
       │                   │                    │
       ▼                   ▼                    ▼
┌─────────────────────────────────────────────────────┐
│                   Foundation Layer                   │
│              (wae-types, wae-macros)                 │
└─────────────────────────────────────────────────────┘
```

每一层只依赖其下层的抽象，确保模块间的解耦和可测试性。
