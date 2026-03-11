//! 数据库事务中间件
//!
//! 提供请求级别的自动事务管理，在请求开始时开启事务，请求成功时提交，失败时回滚。

use crate::connection::{DatabaseConnection, DatabaseResult};
use http::{Request, Response};
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

/// 事务中间件配置
#[derive(Debug, Clone)]
pub struct TransactionConfig {
    /// 是否启用事务
    enabled: bool,
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl TransactionConfig {
    /// 创建新的事务配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否启用事务
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 获取是否启用事务
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// 事务中间件构建器
#[derive(Debug, Clone)]
pub struct TransactionMiddlewareBuilder {
    config: TransactionConfig,
}

impl Default for TransactionMiddlewareBuilder {
    fn default() -> Self {
        Self {
            config: TransactionConfig::default(),
        }
    }
}

impl TransactionMiddlewareBuilder {
    /// 创建新的事务中间件构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置事务配置
    pub fn config(mut self, config: TransactionConfig) -> Self {
        self.config = config;
        self
    }

    /// 设置是否启用事务
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config = self.config.enabled(enabled);
        self
    }

    /// 构建事务中间件层
    pub fn build<C>(self, connection: Arc<C>) -> TransactionLayer<C>
    where
        C: DatabaseConnection + Send + Sync + 'static,
    {
        TransactionLayer {
            config: self.config,
            connection,
        }
    }
}

/// 事务中间件层
#[derive(Debug, Clone)]
pub struct TransactionLayer<C> {
    config: TransactionConfig,
    connection: Arc<C>,
}

impl<C> TransactionLayer<C> {
    /// 创建新的事务中间件层
    pub fn new(config: TransactionConfig, connection: Arc<C>) -> Self {
        Self { config, connection }
    }
}

impl<S, C> Layer<S> for TransactionLayer<C>
where
    C: DatabaseConnection + Send + Sync + Clone + 'static,
{
    type Service = TransactionService<S, C>;

    fn layer(&self, inner: S) -> Self::Service {
        TransactionService {
            inner,
            config: self.config.clone(),
            connection: self.connection.clone(),
        }
    }
}

/// 事务服务
#[derive(Debug, Clone)]
pub struct TransactionService<S, C> {
    inner: S,
    config: TransactionConfig,
    connection: Arc<C>,
}

impl<S, C, ReqBody, ResBody> Service<Request<ReqBody>> for TransactionService<S, C>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Future: Send + 'static,
    C: DatabaseConnection + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = TransactionFuture<S::Future, C>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        req.extensions_mut().insert(self.connection.clone());

        let future = self.inner.call(req);
        TransactionFuture {
            inner: future,
            config: self.config.clone(),
            connection: self.connection.clone(),
            transaction_started: false,
        }
    }
}

pin_project! {
    /// 事务未来
    pub struct TransactionFuture<F, C> {
        #[pin]
        inner: F,
        config: TransactionConfig,
        connection: Arc<C>,
        transaction_started: bool,
    }
}

impl<F, C, Res, E> Future for TransactionFuture<F, C>
where
    F: Future<Output = Result<Res, E>>,
    C: DatabaseConnection + Send + Sync + 'static,
{
    type Output = Result<Res, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        if *this.config.enabled && !*this.transaction_started {
            let connection = this.connection.clone();
            match connection.begin_transaction() {
                Ok(_) => {
                    *this.transaction_started = true;
                }
                Err(_) => {
                    *this.transaction_started = false;
                }
            }
        }

        match this.inner.poll(cx) {
            Poll::Ready(result) => {
                if *this.config.enabled && *this.transaction_started {
                    let connection = this.connection.clone();
                    match result {
                        Ok(_) => {
                            let _ = connection.commit();
                        }
                        Err(_) => {
                            let _ = connection.rollback();
                        }
                    }
                }
                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
