//! HTTP 路由模块
//!
//! 提供路由构建和管理工具。

use crate::Router;
use crate::Handler;
use std::marker::PhantomData;

/// 路由构建器
pub struct RouterBuilder<S = ()> {
    /// 内部路由实例
    router: Router<S>,
    /// 状态类型标记
    _marker: PhantomData<S>,
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
        Self::from_router(Router::with_state(S::default()))
    }

    /// 从现有路由创建构建器
    pub fn from_router(router: Router<S>) -> Self {
        Self { router, _marker: PhantomData }
    }

    /// 嵌套路由
    pub fn nest<T>(mut self, prefix: &str, router: T) -> Self
    where
        T: Into<Router<S>>,
    {
        self.router = self.router.nest_service(prefix, router);
        self
    }

    /// 合并路由
    pub fn merge(mut self, other: Router<S>) -> Self {
        self.router = self.router.merge(other);
        self
    }

    /// 添加 GET 方法路由（使用新的 Handler trait）
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::GET, path.to_string(), handler);
        self
    }

    /// 添加 POST 方法路由（使用新的 Handler trait）
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::POST, path.to_string(), handler);
        self
    }

    /// 添加 PUT 方法路由（使用新的 Handler trait）
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::PUT, path.to_string(), handler);
        self
    }

    /// 添加 DELETE 方法路由（使用新的 Handler trait）
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::DELETE, path.to_string(), handler);
        self
    }

    /// 添加 PATCH 方法路由（使用新的 Handler trait）
    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::PATCH, path.to_string(), handler);
        self
    }

    /// 添加 OPTIONS 方法路由（使用新的 Handler trait）
    pub fn options<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::OPTIONS, path.to_string(), handler);
        self
    }

    /// 添加 HEAD 方法路由（使用新的 Handler trait）
    pub fn head<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::HEAD, path.to_string(), handler);
        self
    }

    /// 添加 TRACE 方法路由（使用新的 Handler trait）
    pub fn trace<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S> + Clone,
        T: 'static,
    {
        self.router.add_route_inner(http::Method::TRACE, path.to_string(), handler);
        self
    }

    /// 构建路由
    pub fn build(self) -> Router<S> {
        self.router
    }

    /// 获取内部 Router 实例
    pub fn into_inner(self) -> Router<S> {
        self.build()
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

/// 方法路由类型
pub struct MethodRouter<S = ()> {
    /// 状态类型标记
    _marker: PhantomData<S>,
}

impl<S> Clone for MethodRouter<S> {
    fn clone(&self) -> Self {
        Self { _marker: PhantomData }
    }
}

impl<S> Default for MethodRouter<S> {
    fn default() -> Self {
        Self { _marker: PhantomData }
    }
}

/// GET 方法路由（使用新的 Handler trait）
pub fn get<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::GET, path.to_string(), handler);
    }
}

/// POST 方法路由（使用新的 Handler trait）
pub fn post<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::POST, path.to_string(), handler);
    }
}

/// PUT 方法路由（使用新的 Handler trait）
pub fn put<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::PUT, path.to_string(), handler);
    }
}

/// DELETE 方法路由（使用新的 Handler trait）
pub fn delete<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::DELETE, path.to_string(), handler);
    }
}

/// PATCH 方法路由（使用新的 Handler trait）
pub fn patch<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::PATCH, path.to_string(), handler);
    }
}

/// OPTIONS 方法路由（使用新的 Handler trait）
pub fn options<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::OPTIONS, path.to_string(), handler);
    }
}

/// HEAD 方法路由（使用新的 Handler trait）
pub fn head<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::HEAD, path.to_string(), handler);
    }
}

/// TRACE 方法路由（使用新的 Handler trait）
pub fn trace<H, T, S>(handler: H) -> impl FnOnce(&mut Router<S>, &str)
where
    H: Handler<T, S> + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |router, path| {
        router.add_route_inner(http::Method::TRACE, path.to_string(), handler);
    }
}

/// 健康检查路由
pub fn health_check_router() -> Router {
    Router::new()
}
