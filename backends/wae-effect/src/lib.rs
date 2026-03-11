//! 代数效应系统
//!
//! 提供声明式的依赖注入能力，允许在处理请求时通过类型安全的接口获取各种依赖。

#![warn(missing_docs)]

use std::{collections::HashMap, sync::Arc};

use http::{Response, StatusCode, request::Parts};
use wae_types::{WaeError, WaeResult};

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
    pub fn get<T: Clone + Send + Sync + 'static>(&self, name: &str) -> WaeResult<T> {
        self.services
            .get(name)
            .and_then(|s| s.downcast_ref::<T>())
            .cloned()
            .ok_or_else(|| WaeError::not_found("Dependency", name))
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
    pub fn get<T: Clone + Send + Sync + 'static>(&self, name: &str) -> WaeResult<T> {
        self.deps.get(name)
    }

    /// 获取请求头
    pub fn header(&self, name: &str) -> Option<&str> {
        self.parts.headers.get(name).and_then(|v| v.to_str().ok())
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

/// WaeError 的包装类型，用于解决 orphan rule 问题
pub struct WaeErrorResponse(pub WaeError);

impl WaeErrorResponse {
    /// 将 WaeError 转换为 http::Response
    pub fn into_response<B>(self) -> Response<B>
    where
        B: From<String>,
    {
        let status = self.0.http_status();
        let body = B::from(self.0.to_string());
        let status_code = StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Response::builder().status(status_code).body(body).unwrap()
    }
}
