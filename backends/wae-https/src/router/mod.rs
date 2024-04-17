//! HTTP 路由模块
//!
//! 提供路由构建和管理工具。

use axum::Router;

/// 路由构建器
pub struct RouterBuilder {
    router: Router,
}

impl RouterBuilder {
    /// 创建新的路由构建器
    pub fn new() -> Self {
        Self { router: Router::new() }
    }

    /// 添加路由
    pub fn route(mut self, path: &str, router: Router) -> Self {
        self.router = self.router.nest(path, router);
        self
    }

    /// 合并路由
    pub fn merge(mut self, router: Router) -> Self {
        self.router = self.router.merge(router);
        self
    }

    /// 构建路由
    pub fn build(self) -> Router {
        self.router
    }
}

impl Default for RouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 健康检查路由
pub fn health_check_router() -> Router {
    Router::new().route("/health", axum::routing::get(|| async { "OK" }))
}
