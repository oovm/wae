//! 代数效应系统
//!
//! 提供声明式的依赖注入能力，允许在处理请求时通过类型安全的接口获取各种依赖。

#![warn(missing_docs)]

use std::{collections::HashMap, fmt, sync::Arc};

use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};

/// 依赖容器
///
/// 存储所有注册的服务实例。
#[derive(Default)]
pub struct Dependencies {
    services: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl Dependencies {
    /// 创建新的依赖容器
    pub fn new() -> Self {
        Self { services: HashMap::new() }
    }

    /// 注册服务
    pub fn register<T: Send + Sync + 'static>(&mut self, name: &str, service: T) {
        self.services.insert(name.to_string(), Box::new(service));
    }

    /// 获取服务
    pub fn get<T: Clone + Send + Sync + 'static>(&self, name: &str) -> Result<T, EffectError> {
        self.services
            .get(name)
            .and_then(|s| s.downcast_ref::<T>())
            .cloned()
            .ok_or_else(|| EffectError::NotFound(name.to_string()))
    }
}

/// 代数效应请求上下文
///
/// 包含依赖容器和请求信息，用于在请求处理过程中获取依赖。
pub struct Effectful {
    deps: Arc<Dependencies>,
    parts: Parts,
}

impl Effectful {
    /// 创建新的 Effectful
    pub fn new(deps: Arc<Dependencies>, parts: Parts) -> Self {
        Self { deps, parts }
    }

    /// 获取依赖
    pub fn get<T: Clone + Send + Sync + 'static>(&self, name: &str) -> Result<T, EffectError> {
        self.deps.get(name)
    }

    /// 获取请求头
    pub fn header(&self, name: &str) -> Option<&str> {
        self.parts.headers.get(name).and_then(|v| v.to_str().ok())
    }
}

/// 从请求中提取 Effectful
impl<S> FromRequestParts<S> for Effectful
where
    S: Send + Sync,
{
    type Rejection = Response<Body>;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let deps = Arc::new(Dependencies::new());
        async move { Ok(Effectful::new(deps, parts.clone())) }
    }
}

/// 代数效应构建器
///
/// 用于构建依赖容器并注册各种依赖。
pub struct AlgebraicEffect {
    deps: Dependencies,
}

impl Default for AlgebraicEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgebraicEffect {
    /// 创建新的代数效应构建器
    pub fn new() -> Self {
        Self { deps: Dependencies::new() }
    }

    /// 注册服务
    pub fn with<T: Send + Sync + 'static>(mut self, name: &str, service: T) -> Self {
        self.deps.register(name, service);
        self
    }

    /// 构建依赖容器
    pub fn build(self) -> Arc<Dependencies> {
        Arc::new(self.deps)
    }
}

/// 效应错误类型
#[derive(Debug)]
pub enum EffectError {
    /// 依赖未找到
    NotFound(String),

    /// 类型转换失败
    TypeMismatch {
        /// 依赖名称
        name: String,
        /// 期望类型
        expected: String,
        /// 实际类型
        actual: String,
    },

    /// 配置加载失败
    ConfigError(wae_config::ConfigError),

    /// 数据库错误
    DatabaseError(String),
}

impl fmt::Display for EffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffectError::NotFound(msg) => write!(f, "Dependency not found: {}", msg),
            EffectError::TypeMismatch { name, expected, actual } => {
                write!(f, "Type mismatch for {}: expected {}, got {}", name, expected, actual)
            }
            EffectError::ConfigError(err) => write!(f, "Config error: {}", err),
            EffectError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for EffectError {}

impl From<wae_config::ConfigError> for EffectError {
    fn from(err: wae_config::ConfigError) -> Self {
        EffectError::ConfigError(err)
    }
}

impl IntoResponse for EffectError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            EffectError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            EffectError::TypeMismatch { .. } => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            EffectError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            EffectError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, message).into_response()
    }
}
