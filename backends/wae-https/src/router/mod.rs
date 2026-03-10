//! HTTP 路由模块
//!
//! 提供路由构建和管理工具。

use axum::{
    Router,
    routing::{delete, get, head, options, patch, post, put, trace, MethodRouter},
};

/// 路由构建器
pub struct RouterBuilder<S = ()> {
    router: Router<S>,
}

impl<S> RouterBuilder<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// 创建新的路由构建器
    pub fn new() -> Self
    where
        S: Default,
    {
        Self { router: Router::new() }
    }

    /// 从现有路由创建构建器
    pub fn from_router(router: Router<S>) -> Self {
        Self { router }
    }

    /// 添加路由处理
    pub fn route(mut self, path: &str, method_router: MethodRouter<S>) -> Self {
        self.router = self.router.route(path, method_router);
        self
    }

    /// 嵌套路由
    pub fn nest(mut self, path: &str, router: Router<S>) -> Self {
        self.router = self.router.nest(path, router);
        self
    }

    /// 合并路由
    pub fn merge(mut self, router: Router<S>) -> Self {
        self.router = self.router.merge(router);
        self
    }

    /// 添加 GET 方法路由
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    /// 添加 POST 方法路由
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, post(handler));
        self
    }

    /// 添加 PUT 方法路由
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, put(handler));
        self
    }

    /// 添加 DELETE 方法路由
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, delete(handler));
        self
    }

    /// 添加 PATCH 方法路由
    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, patch(handler));
        self
    }

    /// 添加 OPTIONS 方法路由
    pub fn options<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, options(handler));
        self
    }

    /// 添加 HEAD 方法路由
    pub fn head<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, head(handler));
        self
    }

    /// 添加 TRACE 方法路由
    pub fn trace<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        self.router = self.router.route(path, trace(handler));
        self
    }

    /// 构建路由
    pub fn build(self) -> Router<S> {
        self.router
    }

    /// 获取内部 axum::Router 实例
    pub fn into_inner(self) -> Router<S> {
        self.router
    }
}

impl<S> Default for RouterBuilder<S>
where
    S: Clone + Send + Sync + 'static + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> From<Router<S>> for RouterBuilder<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn from(router: Router<S>) -> Self {
        Self::from_router(router)
    }
}

impl<S> From<RouterBuilder<S>> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn from(builder: RouterBuilder<S>) -> Self {
        builder.build()
    }
}

/// 健康检查路由
pub fn health_check_router() -> Router {
    Router::new().route("/health", axum::routing::get(|| async { "OK" }))
}
