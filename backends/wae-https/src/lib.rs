#![doc = include_str!("../readme.md")]
#![warn(missing_docs)]

pub mod error;
pub mod extract;
pub mod middleware;
pub mod response;
pub mod router;
pub mod template;
pub mod tls;

pub use wae_session as session;

use axum::{
    Router as AxumRouter,
    body::Body,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use hyper_util::service::TowerToHyperService;
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tracing::info;

pub use wae_types::{CloudError, CloudResult, WaeError, WaeResult};

/// HTTPS 服务结果类型
pub type HttpsResult<T> = WaeResult<T>;

/// HTTPS 服务错误类型
pub type HttpsError = WaeError;

/// HTTP 协议版本配置
#[derive(Debug, Clone, Copy, Default)]
pub enum HttpVersion {
    /// 仅 HTTP/1.1
    Http1Only,
    /// 仅 HTTP/2
    Http2Only,
    /// 自动选择（HTTP/1.1 和 HTTP/2 双协议支持）
    #[default]
    Both,
}

/// HTTP/2 配置选项
#[derive(Debug, Clone)]
pub struct Http2Config {
    /// 是否启用 HTTP/2
    pub enabled: bool,
    /// 是否启用 HTTP/2 推送
    pub enable_push: bool,
    /// 最大并发流数量
    pub max_concurrent_streams: u32,
    /// 初始窗口大小
    pub initial_stream_window_size: u32,
    /// 最大帧大小
    pub max_frame_size: u32,
    /// 启用 CONNECT 协议
    pub enable_connect_protocol: bool,
    /// 流空闲超时时间
    pub stream_idle_timeout: Duration,
}

impl Default for Http2Config {
    fn default() -> Self {
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
    pub fn new() -> Self {
        Self::default()
    }

    /// 禁用 HTTP/2
    pub fn disabled() -> Self {
        Self { enabled: false, ..Self::default() }
    }

    /// 设置是否启用 HTTP/2 推送
    pub fn with_enable_push(mut self, enable: bool) -> Self {
        self.enable_push = enable;
        self
    }

    /// 设置最大并发流数量
    pub fn with_max_concurrent_streams(mut self, max: u32) -> Self {
        self.max_concurrent_streams = max;
        self
    }

    /// 设置初始窗口大小
    pub fn with_initial_stream_window_size(mut self, size: u32) -> Self {
        self.initial_stream_window_size = size;
        self
    }

    /// 设置最大帧大小
    pub fn with_max_frame_size(mut self, size: u32) -> Self {
        self.max_frame_size = size;
        self
    }

    /// 设置是否启用 CONNECT 协议
    pub fn with_enable_connect_protocol(mut self, enable: bool) -> Self {
        self.enable_connect_protocol = enable;
        self
    }

    /// 设置流空闲超时时间
    pub fn with_stream_idle_timeout(mut self, timeout: Duration) -> Self {
        self.stream_idle_timeout = timeout;
        self
    }
}

/// TLS 配置
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// 证书文件路径
    pub cert_path: String,
    /// 私钥文件路径
    pub key_path: String,
}

impl TlsConfig {
    /// 创建新的 TLS 配置
    pub fn new(cert_path: impl Into<String>, key_path: impl Into<String>) -> Self {
        Self { cert_path: cert_path.into(), key_path: key_path.into() }
    }
}

/// HTTPS 服务配置
#[derive(Debug, Clone)]
pub struct HttpsServerConfig {
    /// 服务监听地址
    pub addr: SocketAddr,
    /// 服务名称
    pub service_name: String,
    /// HTTP 协议版本
    pub http_version: HttpVersion,
    /// HTTP/2 配置
    pub http2_config: Http2Config,
    /// TLS 配置（可选）
    pub tls_config: Option<TlsConfig>,
}

impl Default for HttpsServerConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:3000".parse().unwrap(),
            service_name: "wae-https-service".to_string(),
            http_version: HttpVersion::Both,
            http2_config: Http2Config::default(),
            tls_config: None,
        }
    }
}

/// HTTPS 服务构建器
pub struct HttpsServerBuilder {
    config: HttpsServerConfig,
    router: AxumRouter,
}

impl HttpsServerBuilder {
    /// 创建新的服务构建器
    pub fn new() -> Self {
        Self { config: HttpsServerConfig::default(), router: AxumRouter::new() }
    }

    /// 设置监听地址
    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.config.addr = addr;
        self
    }

    /// 设置服务名称
    pub fn service_name(mut self, name: impl Into<String>) -> Self {
        self.config.service_name = name.into();
        self
    }

    /// 设置路由
    pub fn router(mut self, router: AxumRouter) -> Self {
        self.router = router;
        self
    }

    /// 合并路由
    pub fn merge_router(mut self, router: AxumRouter) -> Self {
        self.router = self.router.merge(router);
        self
    }

    /// 设置 HTTP 协议版本
    pub fn http_version(mut self, version: HttpVersion) -> Self {
        self.config.http_version = version;
        self
    }

    /// 设置 HTTP/2 配置
    pub fn http2_config(mut self, config: Http2Config) -> Self {
        self.config.http2_config = config;
        self
    }

    /// 设置 TLS 配置
    pub fn tls(mut self, cert_path: impl Into<String>, key_path: impl Into<String>) -> Self {
        self.config.tls_config = Some(TlsConfig::new(cert_path, key_path));
        self
    }

    /// 设置 TLS 配置对象
    pub fn tls_config(mut self, config: TlsConfig) -> Self {
        self.config.tls_config = Some(config);
        self
    }

    /// 构建 HTTPS 服务
    pub fn build(self) -> HttpsServer {
        HttpsServer { config: self.config, router: self.router }
    }
}

impl Default for HttpsServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTPS 服务
pub struct HttpsServer {
    config: HttpsServerConfig,
    router: AxumRouter,
}

impl HttpsServer {
    /// 启动 HTTPS 服务
    pub async fn serve(self) -> HttpsResult<()> {
        let addr = self.config.addr;
        let service_name = self.config.service_name.clone();
        let protocol_info = self.get_protocol_info();
        let tls_config = self.config.tls_config.clone();

        let listener =
            TcpListener::bind(addr).await.map_err(|e| WaeError::internal(format!("Failed to bind address: {}", e)))?;

        info!("{} {} server starting on {}", service_name, protocol_info, addr);

        match tls_config {
            Some(tls_config) => self.serve_tls(listener, &tls_config).await,
            None => self.serve_plain(listener).await,
        }
    }

    /// 启动纯文本 HTTP 服务（支持 HTTP/1.1 和 h2c）
    async fn serve_plain(self, listener: TcpListener) -> HttpsResult<()> {
        let app = self.router;

        axum::serve(listener, app).await.map_err(|e| WaeError::internal(format!("Server error: {}", e)))?;

        Ok(())
    }

    /// 启动 TLS HTTPS 服务（支持 HTTP/1.1 和 HTTP/2 over TLS）
    async fn serve_tls(self, listener: TcpListener, tls_config: &TlsConfig) -> HttpsResult<()> {
        let tls_acceptor =
            tls::create_tls_acceptor_with_http2(&tls_config.cert_path, &tls_config.key_path, self.config.http2_config.enabled)
                .map_err(|e| WaeError::internal(format!("TLS config error: {}", e)))?;

        let app = self.router;

        loop {
            let (stream, _remote_addr) =
                listener.accept().await.map_err(|e| WaeError::internal(format!("Failed to accept connection: {}", e)))?;

            let acceptor = tls_acceptor.clone();
            let app = app.clone();

            tokio::spawn(async move {
                let tls_stream = match acceptor.accept(stream).await {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::debug!("TLS handshake error: {}", e);
                        return;
                    }
                };

                let service = TowerToHyperService::new(app);
                let io = hyper_util::rt::TokioIo::new(tls_stream);

                let builder = hyper::server::conn::http2::Builder::new(hyper_util::rt::TokioExecutor::new());
                let conn = builder.serve_connection(io, service);

                if let Err(e) = conn.await {
                    tracing::debug!("HTTP/2 connection error: {}", e);
                }
            });
        }
    }

    /// 获取协议信息字符串
    fn get_protocol_info(&self) -> String {
        let tls_info = if self.config.tls_config.is_some() { "S" } else { "" };
        let version_info = match self.config.http_version {
            HttpVersion::Http1Only => "HTTP/1.1",
            HttpVersion::Http2Only => "HTTP/2",
            HttpVersion::Both => "HTTP/1.1+HTTP/2",
        };
        format!("{}{}", version_info, tls_info)
    }
}

/// 统一 JSON 响应结构
#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T> {
    /// 是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<T>,
    /// 错误信息
    pub error: Option<ApiErrorBody>,
    /// 请求追踪 ID
    pub trace_id: Option<String>,
}

/// API 错误体
#[derive(Debug, serde::Serialize)]
pub struct ApiErrorBody {
    /// 错误码
    pub code: String,
    /// 错误消息
    pub message: String,
}

impl<T: serde::Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success { StatusCode::OK } else { StatusCode::BAD_REQUEST };

        let body = serde_json::to_string(&self).unwrap_or_default();
        Response::builder().status(status).header(header::CONTENT_TYPE, "application/json").body(Body::from(body)).unwrap()
    }
}

impl<T> ApiResponse<T>
where
    T: serde::Serialize,
{
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self { success: true, data: Some(data), error: None, trace_id: None }
    }

    /// 创建成功响应（带追踪 ID）
    pub fn success_with_trace(data: T, trace_id: impl Into<String>) -> Self {
        Self { success: true, data: Some(data), error: None, trace_id: Some(trace_id.into()) }
    }

    /// 创建错误响应
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiErrorBody { code: code.into(), message: message.into() }),
            trace_id: None,
        }
    }

    /// 创建错误响应（带追踪 ID）
    pub fn error_with_trace(code: impl Into<String>, message: impl Into<String>, trace_id: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiErrorBody { code: code.into(), message: message.into() }),
            trace_id: Some(trace_id.into()),
        }
    }
}
