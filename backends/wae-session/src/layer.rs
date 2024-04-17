//! Session 中间件层
//!
//! 提供 Session 中间件实现，用于在请求处理过程中管理 Session。

use crate::{Session, SessionConfig, SessionStore};
use axum::{
    body::Body,
    extract::Request,
    http::{Response, header::SET_COOKIE},
    middleware::Next,
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use std::sync::Arc;
use tower::Layer;

/// Session 中间件层
///
/// 用于在 axum 应用中添加 Session 支持。
#[derive(Debug, Clone)]
pub struct SessionLayer<S>
where
    S: SessionStore,
{
    /// Session 存储
    store: S,
    /// Session 配置
    config: SessionConfig,
}

impl<S> SessionLayer<S>
where
    S: SessionStore,
{
    /// 创建新的 Session 中间件层
    pub fn new(store: S, config: SessionConfig) -> Self {
        Self { store, config }
    }

    /// 使用默认配置创建 Session 中间件层
    pub fn with_store(store: S) -> Self {
        Self::new(store, SessionConfig::default())
    }

    /// 生成新的 Session ID
    fn generate_session_id(&self) -> String {
        let mut bytes = vec![0u8; self.config.id_length];
        rand::rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(&bytes)
    }

    /// 从请求中提取 Session ID
    fn extract_session_id(&self, request: &Request) -> Option<String> {
        let cookie_header = request.headers().get("cookie")?.to_str().ok()?;

        for cookie in cookie_header.split(';') {
            let cookie = cookie.trim();
            if let Some(value) = cookie.strip_prefix(&format!("{}=", self.config.cookie_name)) {
                return Some(value.to_string());
            }
        }

        None
    }

    /// 中间件处理函数
    pub async fn middleware(request: Request, next: Next) -> Response<Body> {
        let layer = request.extensions().get::<Arc<Self>>().cloned().expect("SessionLayer not found in extensions");

        let session_id = layer.extract_session_id(&request);
        let session = if let Some(id) = session_id {
            if let Some(data) = layer.store.get(&id).await {
                if let Ok(data_map) = serde_json::from_str(&data) {
                    Session::from_data(id, data_map).await
                }
                else {
                    Session::new(layer.generate_session_id())
                }
            }
            else {
                Session::new(layer.generate_session_id())
            }
        }
        else {
            Session::new(layer.generate_session_id())
        };

        let session = Arc::new(session);
        let mut request = request;
        request.extensions_mut().insert(session.clone());

        let mut response = next.run(request).await;

        if session.is_dirty().await || session.is_new() {
            let data = session.to_json().await;
            layer.store.set(session.id(), &data, layer.config.ttl).await;

            let cookie_value = layer.config.build_cookie_header(session.id());
            if let Ok(header_value) = cookie_value.parse() {
                response.headers_mut().append(SET_COOKIE, header_value);
            }
        }

        response
    }
}

impl<S> Layer<Arc<Self>> for SessionLayer<S>
where
    S: SessionStore,
{
    type Service = SessionService;

    fn layer(&self, _inner: Arc<Self>) -> Self::Service {
        SessionService
    }
}

/// Session 服务
///
/// 内部使用的服务包装器。
pub struct SessionService;

impl tower::Service<Request> for SessionService {
    type Response = Response<Body>;
    type Error = std::convert::Infallible;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request) -> Self::Future {
        Box::pin(async { Ok(Response::new(Body::empty())) })
    }
}
