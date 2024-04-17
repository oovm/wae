# 设计模式

WAE 采用多种设计模式来确保代码的可扩展性、可维护性和可测试性。

## Capability 模式

每个能力定义为独立的 trait，实现关注点分离：

```rust


/// 能力定义

pub trait ChatCapability: Send + Sync {
    async fn chat(&self, params: &ChatParams, config: &AiConfig) -> AiResult<String>;
}


pub trait TextToImageCapability: Send + Sync {
    async fn text_to_image(&self, params: &TextToImageParams, config: &AiConfig) -> AiResult<Vec<String>>;
}


pub trait AiImageCapability: Send + Sync {
    async fn process(&self, input: &AiImageInput, config: &AiConfig) -> AiResult<AiImageOutput>;
}
```

## Provider 模式

服务商通过实现多个 Capability trait 来声明支持的能力：

```rust
pub struct HunyuanProvider;


impl ChatCapability for HunyuanProvider {
    async fn chat(&self, params: &ChatParams, config: &AiConfig) -> AiResult<String> {
        // 腾讯混元对话实现
    }
}


impl TextToImageCapability for HunyuanProvider {
    async fn text_to_image(&self, params: &TextToImageParams, config: &AiConfig) -> AiResult<Vec<String>> {
        // 腾讯混元文生图实现
    }
}

pub struct VolcengineProvider;


impl ChatCapability for VolcengineProvider {
    async fn chat(&self, params: &ChatParams, config: &AiConfig) -> AiResult<String> {
        // 火山引擎对话实现
    }
}
```

## Service 模式

提供便捷的静态方法封装，简化使用：

```rust
pub struct StorageService;

impl StorageService {
    pub fn get_provider(provider_type: &StorageProviderType) -> Box<dyn StorageProvider> {
        match provider_type {
            StorageProviderType::Cos => Box::new(CosProvider),
            StorageProviderType::Oss => Box::new(OssProvider),
            StorageProviderType::Local => Box::new(LocalStorageProvider),
        }
    }

    pub fn sign_url(path: &str, config: &StorageConfig) -> StorageResult<Url> {
        let provider = Self::get_provider(&config.provider);
        provider.sign_url(path, config)
    }

    pub fn get_presigned_put_url(key: &str, config: &StorageConfig) -> StorageResult<Url> {
        let provider = Self::get_provider(&config.provider);
        provider.get_presigned_put_url(key, config)
    }
}
```

## Builder 模式

使用构建器模式创建复杂对象：

```rust
let server = HttpsServerBuilder::new()
    .addr("0.0.0.0:3000".parse()?)
    .service_name("my-service")
    .router(router)
    .http_version(HttpVersion::Both)
    .http2_config(Http2Config::new().with_max_concurrent_streams(256))
    .tls("cert.pem", "key.pem")
    .build();

let pipeline = ResiliencePipelineBuilder::new()
    .with_circuit_breaker(CircuitBreakerConfig::default())
    .with_retry(RetryConfig::default())
    .with_timeout(Duration::from_secs(10))
    .build();
```

## Factory 模式

工厂函数创建实例：

```rust
/// 便捷函数：创建内存认证服务
pub fn memory_auth_service(config: AuthConfig) -> memory::MemoryAuthService {
    memory::MemoryAuthService::new(config)
}

/// 便捷函数：创建 WebSocket 服务端
pub fn websocket_server(config: ServerConfig) -> WebSocketServer {
    WebSocketServer::new(config)
}

/// 便捷函数：创建 WebSocket 客户端
pub fn websocket_client(config: ClientConfig) -> WebSocketClient {
    WebSocketClient::new(config)
}
```

```typescript
// 前端工厂函数
export function createHttpClient(config?: Partial<HttpClientConfig>): HttpClient {
    return new HttpClient(config);
}

export function createAuthClient(config?: AuthClientConfig, storage?: StorageAdapter): AuthClient {
    return new AuthClient(config, storage);
}

export function createWebSocketClient(config: Partial<WebSocketClientConfig> & { url: string }): WebSocketClient {
    return new WebSocketClient(config);
}
```

## Strategy 模式

负载均衡策略：

```rust
pub enum LoadBalanceStrategy {
    RoundRobin,
    Random,
    WeightedRoundRobin,
    LeastConnections,
}

impl LoadBalancer {
    pub fn select(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance> {
        match self.strategy {
            LoadBalanceStrategy::RoundRobin => self.round_robin(instances),
            LoadBalanceStrategy::Random => self.random(instances),
            LoadBalanceStrategy::WeightedRoundRobin => self.weighted_round_robin(instances),
            LoadBalanceStrategy::LeastConnections => self.least_connections(instances),
        }
    }
}
```

## Observer 模式

事件订阅和状态监听：

```rust
// WebSocket 事件处理

pub trait ClientHandler: Send + Sync {
    async fn on_connect(&self, connection: &Connection) -> WebSocketResult<()>;
    async fn on_message(&self, connection: &Connection, message: Message) -> WebSocketResult<()>;
    async fn on_disconnect(&self, connection: &Connection);
}
```

```typescript
// 前端认证状态监听
export type AuthStateListener = (state: AuthState) => void;

export class AuthClient {
    private listeners: Set<AuthStateListener> = new Set();

    subscribe(listener: AuthStateListener): () => void {
        this.listeners.add(listener);
        return () => {
            this.listeners.delete(listener);
        };
    }

    private updateState(partial: Partial<AuthState>): void {
        this.state = { ...this.state, ...partial };
        this.listeners.forEach((listener) => listener(this.state));
    }
}
```

## Repository 模式

数据访问抽象：

```rust

pub trait ServiceRegistry: Send + Sync {
    async fn register(&self, instance: &ServiceInstance) -> ServiceResult<()>;
    async fn deregister(&self, service_id: &str) -> ServiceResult<()>;
    async fn heartbeat(&self, service_id: &str) -> ServiceResult<()>;
    async fn update_status(&self, service_id: &str, status: ServiceStatus) -> ServiceResult<()>;
}


pub trait ServiceDiscovery: Send + Sync {
    async fn discover(&self, service_name: &str) -> ServiceResult<Vec<ServiceInstance>>;
    async fn discover_one(&self, service_name: &str) -> ServiceResult<Option<ServiceInstance>>;
    async fn subscribe(&self, service_name: &str) -> ServiceResult<Box<dyn ServiceSubscription>>;
}
```

## Pipeline 模式

弹性管道组合多种容错策略：

```rust
pub struct ResiliencePipeline {
    circuit_breaker: Option<Arc<CircuitBreaker>>,
    rate_limiter: Option<Arc<RateLimiter>>,
    retry_config: Option<RetryConfig>,
    timeout: Option<Duration>,
    bulkhead: Option<Arc<Bulkhead>>,
}

impl ResiliencePipeline {
    pub async fn execute<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: Future<Output = Result<T, E>>,
    {
        // 依次应用：超时 -> 熔断 -> 限流 -> 舱壁 -> 重试
        let future = self.apply_timeout(f);
        let future = self.apply_circuit_breaker(future);
        let future = self.apply_rate_limiter(future);
        let future = self.apply_bulkhead(future);
        self.apply_retry(future).await
    }
}
```

## Interceptor 模式

HTTP 请求/响应拦截：

```typescript
export type RequestInterceptor = (
    url: string,
    options: RequestInit,
) => Promise<{ url: string; options: RequestInit }> | { url: string; options: RequestInit };

export type ResponseInterceptor = <T>(
    response: ApiResponse<T>,
    request: { url: string; options: RequestInit },
) => Promise<ApiResponse<T>> | ApiResponse<T>;

export type ErrorInterceptor = (
    error: CloudError,
    request: { url: string; options: RequestInit },
) => Promise<void> | void;

export class HttpClient {
    addRequestInterceptor(interceptor: RequestInterceptor): void;
    addResponseInterceptor(interceptor: ResponseInterceptor): void;
    addErrorInterceptor(interceptor: ErrorInterceptor): void;
}
```

## 扩展指南

### 添加新的 AI 服务商

1. 在 `wae-ai/src/providers/` 创建新文件
2. 实现需要的 Capability trait
3. 在 `providers/mod.rs` 中导出

```rust
// wae-ai/src/providers/new_provider.rs
use crate::{AiConfig, AiResult};
use crate::capabilities::{ChatCapability, ChatParams};

pub struct NewProvider;


impl ChatCapability for NewProvider {
    async fn chat(&self, params: &ChatParams, config: &AiConfig) -> AiResult<String> {
        // 实现调用逻辑
    }
}
```

### 添加新的存储服务商

1. 在 `wae-storage/src/providers/` 创建新文件
2. 实现 `StorageProvider` trait
3. 在 `StorageProviderType` 枚举中添加新类型

```rust
// wae-storage/src/providers/new_provider.rs
use crate::{StorageConfig, StorageResult, StorageProvider};
use url::Url;

pub struct NewProvider;

impl StorageProvider for NewProvider {
    fn get_presigned_put_url(&self, key: &str, config: &StorageConfig) -> StorageResult<Url> {
        // 实现预签名 URL 生成
    }

    fn sign_url(&self, path: &str, config: &StorageConfig) -> StorageResult<Url> {
        // 实现签名 URL 生成
    }
}
```

### 添加新的能力

1. 在 `wae-ai/src/capabilities/` 创建新文件
2. 定义 trait 和参数/返回类型
3. 为现有服务商实现该能力

```rust
// wae-ai/src/capabilities/embedding.rs

use crate::{AiConfig, AiResult};

pub struct EmbeddingParams {
    pub text: String,
    pub model: Option<String>,
}

pub struct EmbeddingResult {
    pub embedding: Vec<f32>,
    pub dimensions: usize,
}


pub trait EmbeddingCapability: Send + Sync {
    async fn embed(&self, params: &EmbeddingParams, config: &AiConfig) -> AiResult<EmbeddingResult>;
}
```
