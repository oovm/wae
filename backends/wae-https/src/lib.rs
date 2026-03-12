#![doc = include_str!("readme.md")]
#![warn(missing_docs)]

pub mod error;
pub mod extract;
pub mod middleware;
pub mod response;
pub mod router;
pub mod template;
pub mod tls;

pub use wae_session as session;

pub use router::{RouterBuilder, MethodRouter, get, post, put, delete, patch, options, head, trace};
pub use response::{JsonResponse, Html, Redirect, Attachment, StreamResponse};

use http::{Response, StatusCode, header};
use http_body_util::Full;
use hyper::body::Bytes;
use std::{net::SocketAddr, path::Path, time::Duration};
use tokio::net::TcpListener;
use tracing::info;

pub use wae_types::{WaeError, WaeResult};

/// HTTP 响应体类型
pub type Body = Full<Bytes>;

/// 创建空的 Body
pub fn empty_body() -&gt; Body {
    Full::new(Bytes::new())
}

/// 创建带内容的 Body
pub fn full_body&lt;B: Into&lt;Bytes&gt;&gt;(data: B) -&gt; Body {
    Full::new(data.into())
}

/// HTTPS 操作结果类型
pub type HttpsResult&lt;T&gt; = WaeResult&lt;T&gt;;

/// HTTPS 错误类型
pub type HttpsError = WaeError;

/// 将类型转换为 HTTP 响应的 trait
///
/// 类似于 Axum 的 IntoResponse trait，用于将各种类型转换为 HTTP 响应。
pub trait IntoResponse {
    /// 将自身转换为 HTTP 响应
    fn into_response(self) -&gt; Response&lt;Body&gt;;
}

impl IntoResponse for Response&lt;Body&gt; {
    fn into_response(self) -&gt; Response&lt;Body&gt; {
        self
    }
}

impl IntoResponse for &amp;'static str {
    fn into_response(self) -&gt; Response&lt;Body&gt; {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(full_body(self))
            .unwrap()
    }
}

impl IntoResponse for String {
    fn into_response(self) -&gt; Response&lt;Body&gt; {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(full_body(self))
            .unwrap()
    }
}

impl&lt;T: IntoResponse&gt; IntoResponse for (StatusCode, T) {
    fn into_response(self) -&gt; Response&lt;Body&gt; {
        let mut res = self.1.into_response();
        *res.status_mut() = self.0;
        res
    }
}

/// 处理函数 trait，类似于 Axum 的 Handler trait
///
/// 定义了如何将异步函数与提取器绑定并调用。
pub trait Handler&lt;T, S&gt;: Clone + Send + Sync + 'static {
    /// 调用处理函数
    ///
    /// # 参数
    ///
    /// * `parts` - 请求上下文
    /// * `state` - 应用状态
    async fn call(self, parts: crate::extract::RequestParts, state: S) -&gt; Response&lt;Body&gt;;
}

/// 元组处理函数支持（最多支持 1 个参数）
macro_rules! impl_handler {
    (
        [$($ty:ident),*],
        $last:ident
    ) =&gt; {
        #[allow(non_snake_case, unused_mut)]
        impl&lt;F, Fut, S, $($ty,)* $last&gt; Handler&lt;($($ty,)* $last,), S&gt; for F
        where
            F: FnOnce($($ty,)* $last,) -&gt; Fut + Clone + Send + Sync + 'static,
            Fut: std::future::Future&lt;Output = Response&lt;Body&gt;&gt; + Send,
            S: Clone + Send + Sync + 'static,
            $($ty: crate::extract::FromRequestParts&lt;S, Error = crate::extract::ExtractorError&gt;,)*
            $last: crate::extract::FromRequestParts&lt;S, Error = crate::extract::ExtractorError&gt;,
        {
            async fn call(self, parts: crate::extract::RequestParts, state: S) -&gt; Response&lt;Body&gt; {
                let result: Response&lt;Body&gt; = match &lt;($($ty,)* $last,) as crate::extract::FromRequestParts&lt;S&gt;&gt;::from_request_parts(&amp;parts, &amp;state).await {
                    Ok(($($ty,)* $last,)) =&gt; {
                        self($($ty,)* $last,).await
                    }
                    Err(e) =&gt; {
                        let error_msg = e.to_string();
                        Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
                            .body(full_body(error_msg))
                            .unwrap()
                    }
                };
                result
            }
        }
    };
}

impl_handler!([], T1);

/// 处理函数包装器 trait
trait HandlerWrapper&lt;S&gt;: Send + Sync + 'static {
    /// 调用处理函数
    fn call(&amp;self, parts: crate::extract::RequestParts, state: S) -&gt; std::pin::Pin&lt;Box&lt;dyn std::future::Future&lt;Output = Response&lt;Body&gt;&gt; + Send&gt;&gt;;
    /// 克隆处理函数包装器
    fn clone_box(&amp;self) -&gt; Box&lt;dyn HandlerWrapper&lt;S&gt;&gt;;
}

impl&lt;S: Clone + 'static&gt; Clone for Box&lt;dyn HandlerWrapper&lt;S&gt;&gt; {
    fn clone(&amp;self) -&gt; Self {
        self.clone_box()
    }
}

/// 处理函数包装器实现
struct HandlerWrapperImpl&lt;H, T, S&gt; {
    handler: H,
    _marker: std::marker::PhantomData&lt;(T, S)&gt;,
}

impl&lt;H, T, S&gt; Clone for HandlerWrapperImpl&lt;H, T, S&gt;
where
    H: Clone,
{
    fn clone(&amp;self) -&gt; Self {
        Self {
            handler: self.handler.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl&lt;H, T, S&gt; HandlerWrapper&lt;S&gt; for HandlerWrapperImpl&lt;H, T, S&gt;
where
    H: Handler&lt;T, S&gt; + Clone,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    fn call(&amp;self, parts: crate::extract::RequestParts, state: S) -&gt; std::pin::Pin&lt;Box&lt;dyn std::future::Future&lt;Output = Response&lt;Body&gt;&gt; + Send&gt;&gt; {
        let handler = self.handler.clone();
        Box::pin(async move {
            handler.call(parts, state).await
        })
    }

    fn clone_box(&amp;self) -&gt; Box&lt;dyn HandlerWrapper&lt;S&gt;&gt; {
        Box::new(self.clone())
    }
}

/// 自定义路由类型
pub struct Router&lt;S = ()&gt; {
    routes: std::collections::HashMap&lt;http::Method, matchit::Router&lt;Box&lt;dyn HandlerWrapper&lt;S&gt;&gt;&gt;&gt;,
    raw_routes: Vec&lt;RouteEntry&lt;S&gt;&gt;,
    state: S,
}

/// 路由条目
struct RouteEntry&lt;S&gt; {
    method: http::Method,
    path: String,
    handler: Box&lt;dyn HandlerWrapper&lt;S&gt;&gt;,
}

impl&lt;S: Clone&gt; Clone for Router&lt;S&gt; {
    fn clone(&amp;self) -&gt; Self {
        let mut routes = std::collections::HashMap::new();
        for (method, router) in &amp;self.routes {
            let mut new_router = matchit::Router::new();
            for (path, handler) in router.iter() {
                let _ = new_router.insert(path, handler.clone());
            }
            routes.insert(method.clone(), new_router);
        }
        Self {
            routes,
            raw_routes: self.raw_routes.clone(),
            state: self.state.clone(),
        }
    }
}

impl&lt;S: Clone&gt; Clone for RouteEntry&lt;S&gt; {
    fn clone(&amp;self) -&gt; Self {
        Self {
            method: self.method.clone(),
            path: self.path.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl Default for Router&lt;()&gt; {
    fn default() -&gt; Self {
        Self::new()
    }
}

impl Router&lt;()&gt; {
    /// 创建新的空路由
    pub fn new() -&gt; Self {
        Self { routes: std::collections::HashMap::new(), raw_routes: Vec::new(), state: () }
    }
}

impl&lt;S&gt; Router&lt;S&gt; {
    /// 创建带状态的路由
    pub fn with_state(state: S) -&gt; Self {
        Self { routes: std::collections::HashMap::new(), raw_routes: Vec::new(), state }
    }

    /// 获取路由状态
    pub fn state(&amp;self) -&gt; &amp;S {
        &amp;self.state
    }

    /// 获取可变的路由状态
    pub fn state_mut(&amp;mut self) -&gt; &amp;mut S {
        &amp;mut self.state
    }

    /// 添加路由到路由表
    pub fn add_route_inner(
        &amp;mut self,
        method: http::Method,
        path: String,
        handler: Box&lt;dyn std::any::Any + Send + Sync + 'static&gt;,
    ) {
        panic!("add_route_inner should not be called directly, use add_route instead");
    }

    /// 添加路由到路由表（内部使用）
    fn add_route_with_wrapper(
        &amp;mut self,
        method: http::Method,
        path: String,
        handler: Box&lt;dyn HandlerWrapper&lt;S&gt;&gt;,
    ) {
        let entry = RouteEntry { method: method.clone(), path: path.clone(), handler: handler.clone() };
        self.raw_routes.push(entry);

        let router = self.routes.entry(method).or_insert_with(matchit::Router::new);
        let _ = router.insert(path, handler);
    }
}

impl&lt;S&gt; Router&lt;S&gt;
where
    S: Clone + Send + Sync + 'static,
{
    /// 添加路由
    pub fn add_route&lt;H, T&gt;(
        &amp;mut self,
        method: http::Method,
        path: &amp;str,
        handler: H,
    ) where
        H: Handler&lt;T, S&gt; + Clone,
        T: 'static,
    {
        let wrapper = HandlerWrapperImpl {
            handler,
            _marker: std::marker::PhantomData,
        };
        self.add_route_with_wrapper(method, path.to_string(), Box::new(wrapper));
    }

    /// 合并另一个路由
    pub fn merge(mut self, other: Router&lt;S&gt;) -&gt; Self {
        for entry in other.raw_routes {
            self.add_route_with_wrapper(entry.method, entry.path, entry.handler);
        }
        self
    }

    /// 嵌套服务
    pub fn nest_service&lt;T&gt;(mut self, prefix: &amp;str, service: T) -&gt; Self
    where
        T: Into&lt;Router&lt;S&gt;&gt;,
    {
        let other = service.into();
        for entry in other.raw_routes {
            let new_path = format!("{}{}", prefix.trim_end_matches('/'), entry.path);
            self.add_route_with_wrapper(entry.method, new_path, entry.handler);
        }
        self
    }
}

/// HTTP 版本配置
///
/// 用于配置服务器支持的 HTTP 协议版本。
#[derive(Debug, Clone, Copy, Default)]
pub enum HttpVersion {
    /// 仅支持 HTTP/1.1
    Http1Only,
    /// 仅支持 HTTP/2
    Http2Only,
    /// 同时支持 HTTP/1.1 和 HTTP/2
    #[default]
    Both,
    /// HTTP/3 QUIC 支持
    Http3,
}

/// HTTP/2 配置
///
/// 用于配置 HTTP/2 协议的各项参数。
#[derive(Debug, Clone)]
pub struct Http2Config {
    /// 是否启用 HTTP/2
    pub enabled: bool,
    /// 是否启用服务器推送
    pub enable_push: bool,
    /// 最大并发流数量
    pub max_concurrent_streams: u32,
    /// 初始流窗口大小
    pub initial_stream_window_size: u32,
    /// 最大帧大小
    pub max_frame_size: u32,
    /// 是否启用 CONNECT 协议扩展
    pub enable_connect_protocol: bool,
    /// 流空闲超时时间
    pub stream_idle_timeout: Duration,
}

impl Default for Http2Config {
    fn default() -&gt; Self {
        Self {
            enabled: true,
            enable_push: false,
            max_concurrent_streams: 256,
            initial_stream_window_size: 65535,
            max_frame_size: 16384,
            enable_connect_protocol: false,
            stream_idle_timeout: Duration::from_secs(60),
        }
    }
}

impl Http2Config {
    /// 创建默认的 HTTP/2 配置
    pub fn new() -&gt; Self {
        Self::default()
    }

    /// 创建禁用 HTTP/2 的配置
    pub fn disabled() -&gt; Self {
        Self { enabled: false, ..Self::default() }
    }

    /// 设置是否启用服务器推送
    pub fn with_enable_push(mut self, enable: bool) -&gt; Self {
        self.enable_push = enable;
        self
    }

    /// 设置最大并发流数量
    pub fn with_max_concurrent_streams(mut self, max: u32) -&gt; Self {
        self.max_concurrent_streams = max;
        self
    }

    /// 设置初始流窗口大小
    pub fn with_initial_stream_window_size(mut self, size: u32) -&gt; Self {
        self.initial_stream_window_size = size;
        self
    }

    /// 设置最大帧大小
    pub fn with_max_frame_size(mut self, size: u32) -&gt; Self {
        self.max_frame_size = size;
        self
    }

    /// 设置是否启用 CONNECT 协议扩展
    pub fn with_enable_connect_protocol(mut self, enable: bool) -&gt; Self {
        self.enable_connect_protocol = enable;
        self
    }

    /// 设置流空闲超时时间
    pub fn with_stream_idle_timeout(mut self, timeout: Duration) -&gt; Self {
        self.stream_idle_timeout = timeout;
        self
    }
}

/// TLS 配置
///
/// 用于配置 TLS 证书和密钥。
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// 证书文件路径
    pub cert_path: String,
    /// 私钥文件路径
    pub key_path: String,
}

impl TlsConfig {
    /// 创建新的 TLS 配置
    ///
    /// # 参数
    ///
    /// * `cert_path` - 证书文件路径
    /// * `key_path` - 私钥文件路径
    pub fn new(cert_path: impl Into&lt;String&gt;, key_path: impl Into&lt;String&gt;) -&gt; Self {
        Self { cert_path: cert_path.into(), key_path: key_path.into() }
    }
}

/// HTTP/3 QUIC 配置
///
/// 用于配置 HTTP/3 QUIC 协议的设置。
#[derive(Debug, Clone, Default)]
pub struct Http3Config {
    /// 是否启用 HTTP/3 QUIC 支持
    pub enabled: bool,
}

impl Http3Config {
    /// 创建默认的 HTTP/3 配置
    pub fn new() -&gt; Self {
        Self::default()
    }

    /// 创建启用 HTTP/3 的配置
    pub fn enabled() -&gt; Self {
        Self { enabled: true }
    }
}

/// HTTPS 服务器配置
///
/// 用于配置 HTTPS 服务器的各项参数。
#[derive(Debug, Clone)]
pub struct HttpsServerConfig {
    /// 服务器监听地址
    pub addr: SocketAddr,
    /// 服务名称
    pub service_name: String,
    /// HTTP 版本配置
    pub http_version: HttpVersion,
    /// HTTP/2 配置
    pub http2_config: Http2Config,
    /// HTTP/3 配置
    pub http3_config: Http3Config,
    /// TLS 配置
    pub tls_config: Option&lt;TlsConfig&gt;,
}

impl Default for HttpsServerConfig {
    fn default() -&gt; Self {
        Self {
            addr: "0.0.0.0:3000".parse().unwrap(),
            service_name: "wae-https-service".to_string(),
            http_version: HttpVersion::Both,
            http2_config: Http2Config::default(),
            http3_config: Http3Config::default(),
            tls_config: None,
        }
    }
}

/// HTTPS 服务器构建器
///
/// 用于构建和配置 HTTPS 服务器。
pub struct HttpsServerBuilder&lt;S = ()&gt; {
    config: HttpsServerConfig,
    router: Router&lt;S&gt;,
    _marker: std::marker::PhantomData&lt;S&gt;,
}

impl HttpsServerBuilder&lt;()&gt; {
    /// 创建新的 HTTPS 服务器构建器
    pub fn new() -&gt; Self {
        Self { config: HttpsServerConfig::default(), router: Router::new(), _marker: std::marker::PhantomData }
    }
}

impl Default for HttpsServerBuilder&lt;()&gt; {
    fn default() -&gt; Self {
        Self::new()
    }
}

impl&lt;S&gt; HttpsServerBuilder&lt;S&gt;
where
    S: Clone + Send + Sync + 'static,
{
    /// 设置服务器监听地址
    pub fn addr(mut self, addr: SocketAddr) -&gt; Self {
        self.config.addr = addr;
        self
    }

    /// 设置服务名称
    pub fn service_name(mut self, name: impl Into&lt;String&gt;) -&gt; Self {
        self.config.service_name = name.into();
        self
    }

    /// 设置路由
    pub fn router&lt;T&gt;(mut self, router: T) -&gt; Self
    where
        T: Into&lt;Router&lt;S&gt;&gt;,
    {
        self.router = router.into();
        self
    }

    /// 合并路由
    pub fn merge_router(mut self, router: Router&lt;S&gt;) -&gt; Self {
        self.router = self.router.merge(router);
        self
    }

    /// 设置 HTTP 版本配置
    pub fn http_version(mut self, version: HttpVersion) -&gt; Self {
        self.config.http_version = version;
        self
    }

    /// 设置 HTTP/2 配置
    pub fn http2_config(mut self, config: Http2Config) -&gt; Self {
        self.config.http2_config = config;
        self
    }

    /// 设置 HTTP/3 配置
    pub fn http3_config(mut self, config: Http3Config) -&gt; Self {
        self.config.http3_config = config;
        self
    }

    /// 设置 TLS 证书和密钥
    ///
    /// # 参数
    ///
    /// * `cert_path` - 证书文件路径
    /// * `key_path` - 私钥文件路径
    pub fn tls(mut self, cert_path: impl Into&lt;String&gt;, key_path: impl Into&lt;String&gt;) -&gt; Self {
        self.config.tls_config = Some(TlsConfig::new(cert_path, key_path));
        self
    }

    /// 设置 TLS 配置
    pub fn tls_config(mut self, config: TlsConfig) -&gt; Self {
        self.config.tls_config = Some(config);
        self
    }

    /// 构建 HTTPS 服务器
    pub fn build(self) -&gt; HttpsServer&lt;S&gt; {
        HttpsServer { config: self.config, router: self.router, _marker: std::marker::PhantomData }
    }
}

/// HTTPS 服务器
///
/// 提供 HTTP/HTTPS 服务的核心类型。
pub struct HttpsServer&lt;S = ()&gt; {
    config: HttpsServerConfig,
    router: Router&lt;S&gt;,
    _marker: std::marker::PhantomData&lt;S&gt;,
}

impl&lt;S&gt; HttpsServer&lt;S&gt;
where
    S: Clone + Send + Sync + 'static,
{
    /// 启动服务器
    pub async fn serve(self) -&gt; HttpsResult&lt;()&gt; {
        let addr = self.config.addr;
        let service_name = self.config.service_name.clone();
        let protocol_info = self.get_protocol_info();
        let tls_config = self.config.tls_config.clone();

        let listener =
            TcpListener::bind(addr).await.map_err(|e| WaeError::internal(format!("Failed to bind address: {}", e)))?;

        info!("{} {} server starting on {}", service_name, protocol_info, addr);

        match tls_config {
            Some(tls_config) =&gt; self.serve_tls(listener, &amp;tls_config).await,
            None =&gt; self.serve_plain(listener).await,
        }
    }

    /// 启动 HTTP 服务器
    async fn serve_plain(self, listener: TcpListener) -&gt; HttpsResult&lt;()&gt; {
        loop {
            let (stream, _addr) = listener
                .accept()
                .await
                .map_err(|e| WaeError::internal(format!("Accept error: {}", e)))?;

            let router = self.router.clone();
            tokio::spawn(async move {
                let service = RouterService::new(router);
                let io = hyper_util::rt::tokio::TokioIo::new(stream);
                let _ = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, service)
                    .await;
            });
        }
    }

    /// 启动 HTTPS 服务器
    async fn serve_tls(self, listener: TcpListener, tls_config: &amp;TlsConfig) -&gt; HttpsResult&lt;()&gt; {
        let enable_http2 = matches!(
            self.config.http_version,
            HttpVersion::Http2Only | HttpVersion::Both
        );

        let acceptor = crate::tls::create_tls_acceptor_with_http2(
            &amp;tls_config.cert_path,
            &amp;tls_config.key_path,
            enable_http2,
        )?;

        loop {
            let (stream, _addr) = listener
                .accept()
                .await
                .map_err(|e| WaeError::internal(format!("Accept error: {}", e)))?;

            let acceptor = acceptor.clone();
            let router = self.router.clone();

            tokio::spawn(async move {
                let tls_stream = match acceptor.accept(stream).await {
                    Ok(s) =&gt; s,
                    Err(e) =&gt; {
                        tracing::error!("TLS handshake error: {}", e);
                        return;
                    }
                };

                let service = RouterService::new(router);
                let io = hyper_util::rt::tokio::TokioIo::new(tls_stream);
                let _ = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, service)
                    .await;
            });
        }
    }

    fn get_protocol_info(&amp;self) -&gt; String {
        let tls_info = if self.config.tls_config.is_some() { "S" } else { "" };
        let version_info = match self.config.http_version {
            HttpVersion::Http1Only =&gt; "HTTP/1.1",
            HttpVersion::Http2Only =&gt; "HTTP/2",
            HttpVersion::Both =&gt; "HTTP/1.1+HTTP/2",
            HttpVersion::Http3 =&gt; "HTTP/3",
        };
        format!("{}{}", version_info, tls_info)
    }
}

/// API 响应结构
///
/// 用于标准化 API 响应格式。
#[derive(Debug, serde::Serialize)]
pub struct ApiResponse&lt;T&gt; {
    /// 是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option&lt;T&gt;,
    /// 错误信息
    pub error: Option&lt;ApiErrorBody&gt;,
    /// 追踪 ID
    pub trace_id: Option&lt;String&gt;,
}

/// API 错误响应结构
///
/// 用于标准化 API 错误响应格式。
#[derive(Debug, serde::Serialize)]
pub struct ApiErrorBody {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
}

impl&lt;T: serde::Serialize&gt; ApiResponse&lt;T&gt; {
    /// 将 API 响应转换为 HTTP 响应
    pub fn into_response(self) -&gt; Response&lt;Body&gt; {
        let status = if self.success { StatusCode::OK } else { StatusCode::BAD_REQUEST };
        let body = serde_json::to_string(&amp;self).unwrap_or_default();
        Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body)))
            .unwrap()
    }
}

impl&lt;T&gt; IntoResponse for ApiResponse&lt;T&gt;
where
    T: serde::Serialize