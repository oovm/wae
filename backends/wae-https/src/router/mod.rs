//! HTTP 路由模块
//!
//! 提供路由构建和管理工具。

use crate::Router;
use std::marker::PhantomData;

/// 路由构建器
pub struct RouterBuilder<S = ()> {
    /// 内部路由实例
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
        Self::from_router(Router::with_state(S::default()))
    }

    /// 从现有路由创建构建器
    pub fn from_router(router: Router<S>) -> Self {
        Self { router }
    }

    /// 添加路由处理
    pub fn route(mut self, path: &str, method_router: MethodRouter<S>) -> Self {
        for (method, handler) in method_router.handlers {
            self.router.add_route_inner(method, path.to_string(), handler);
        }
        self
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

    /// 添加 GET 方法路由
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::GET, path.to_string(), Box::new(handler));
        self
    }

    /// 添加 POST 方法路由
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::POST, path.to_string(), Box::new(handler));
        self
    }

    /// 添加 PUT 方法路由
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::PUT, path.to_string(), Box::new(handler));
        self
    }

    /// 添加 DELETE 方法路由
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::DELETE, path.to_string(), Box::new(handler));
        self
    }

    /// 添加 PATCH 方法路由
    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::PATCH, path.to_string(), Box::new(handler));
        self
    }

    /// 添加 OPTIONS 方法路由
    pub fn options<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::OPTIONS, path.to_string(), Box::new(handler));
        self
    }

    /// 添加 HEAD 方法路由
    pub fn head<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::HEAD, path.to_string(), Box::new(handler));
        self
    }

    /// 添加 TRACE 方法路由
    pub fn trace<H, T>(mut self, path: &str, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.router.add_route_inner(http::Method::TRACE, path.to_string(), Box::new(handler));
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
    /// 存储的处理函数
    handlers: Vec<(http::Method, Box<dyn std::any::Any + Send + Sync + 'static>)>,
    /// 状态类型标记
    _marker: PhantomData<S>,
}

impl<S> Clone for MethodRouter<S> {
    fn clone(&self) -> Self {
        Self {
            handlers: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<S> Default for MethodRouter<S> {
    fn default() -> Self {
        Self {
            handlers: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<S> MethodRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// 添加 GET 方法处理
    pub fn get<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::GET, Box::new(handler)));
        self
    }

    /// 添加 POST 方法处理
    pub fn post<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::POST, Box::new(handler)));
        self
    }

    /// 添加 PUT 方法处理
    pub fn put<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::PUT, Box::new(handler)));
        self
    }

    /// 添加 DELETE 方法处理
    pub fn delete<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::DELETE, Box::new(handler)));
        self
    }

    /// 添加 PATCH 方法处理
    pub fn patch<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::PATCH, Box::new(handler)));
        self
    }

    /// 添加 OPTIONS 方法处理
    pub fn options<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::OPTIONS, Box::new(handler)));
        self
    }

    /// 添加 HEAD 方法处理
    pub fn head<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::HEAD, Box::new(handler)));
        self
    }

    /// 添加 TRACE 方法处理
    pub fn trace<H, T>(mut self, handler: H) -> Self
    where
        T: 'static,
        H: Send + Sync + 'static,
    {
        self.handlers.push((http::Method::TRACE, Box::new(handler)));
        self
    }
}

/// GET 方法路由
pub fn get<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().get::<H, T>(handler)
}

/// POST 方法路由
pub fn post<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().post::<H, T>(handler)
}

/// PUT 方法路由
pub fn put<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().put::<H, T>(handler)
}

/// DELETE 方法路由
pub fn delete<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().delete::<H, T>(handler)
}

/// PATCH 方法路由
pub fn patch<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().patch::<H, T>(handler)
}

/// OPTIONS 方法路由
pub fn options<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().options::<H, T>(handler)
}

/// HEAD 方法路由
pub fn head<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().head::<H, T>(handler)
}

/// TRACE 方法路由
pub fn trace<H, T, S>(handler: H) -> MethodRouter<S>
where
    T: 'static,
    H: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    MethodRouter::default().trace::<H, T>(handler)
}

/// 健康检查路由
pub fn health_check_router() -> Router {
    Router::new()
}