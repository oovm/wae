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
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use hyper_util::service::TowerToHyperService;
use std::{net::SocketAddr, path::Path, time::Duration};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

pub use wae_types::{WaeError, WaeResult};

/// HTTP 响应体类型
pub type Body = axum::body::Body;

/// 创建空的 Body
pub fn empty_body() -> Body {
    Body::empty()
}

/// 创建带内容的 Body
pub fn full_body<B: Into<Body>>(data: B) -> Body {
    data.into()
}

pub type HttpsResult<T> = WaeResult<T>;
pub type HttpsError = WaeError;

#[derive(Debug, Clone, Copy, Default)]
pub enum HttpVersion {
    Http1Only,
    Http2Only,
    #[default]
    Both,
    /// HTTP/3 QUIC 支持
    Http3,
}

#[derive(Debug, Clone)]
pub struct Http2Config {
    pub enabled: bool,
    pub enable_push: bool,
    pub max_concurrent_streams: u32,
    pub initial_stream_window_size: u32,
    pub max_frame_size: u32,
    pub enable_connect_protocol: bool,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled() -> Self {
        Self { enabled: false, ..Self::default() }
    }

    pub fn with_enable_push(mut self, enable: bool) -> Self {
        self.enable_push = enable;
        self
    }

    pub fn with_max_concurrent_streams(mut self, max: u32) -> Self {
        self.max_concurrent_streams = max;
        self
    }

    pub fn with_initial_stream_window_size(mut self, size: u32) -> Self {
        self.initial_stream_window_size = size;
        self
    }

    pub fn with_max_frame_size(mut self, size: u32) -> Self {
        self.max_frame_size = size;
        self
    }

    pub fn with_enable_connect_protocol(mut self, enable: bool) -> Self {
        self.enable_connect_protocol = enable;
        self
    }

    pub fn with_stream_idle_timeout(mut self, timeout: Duration) -> Self {
        self.stream_idle_timeout = timeout;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}

impl TlsConfig {
    pub fn new(cert_path: impl Into<String>, key_path: impl Into<String>) -> Self {
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
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建启用 HTTP/3 的配置
    pub fn enabled() -> Self {
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
    pub tls_config: Option<TlsConfig>,
}

impl Default for HttpsServerConfig {
    fn default() -> Self {
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

pub struct HttpsServerBuilder {
    config: HttpsServerConfig,
    router: AxumRouter,
}

impl HttpsServerBuilder {
    pub fn new() -> Self {
        Self { config: HttpsServerConfig::default(), router: AxumRouter::new() }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.config.addr = addr;
        self
    }

    pub fn service_name(mut self, name: impl Into<String>) -> Self {
        self.config.service_name = name.into();
        self
    }

    pub fn router(mut self, router: AxumRouter) -> Self {
        self.router = router;
        self
    }

    pub fn merge_router(mut self, router: AxumRouter) -> Self {
        self.router = self.router.merge(router);
        self
    }

    pub fn http_version(mut self, version: HttpVersion) -> Self {
        self.config.http_version = version;
        self
    }

    pub fn http2_config(mut self, config: Http2Config) -> Self {
        self.config.http2_config = config;
        self
    }

    pub fn http3_config(mut self, config: Http3Config) -> Self {
        self.config.http3_config = config;
        self
    }

    pub fn tls(mut self, cert_path: impl Into<String>, key_path: impl Into<String>) -> Self {
        self.config.tls_config = Some(TlsConfig::new(cert_path, key_path));
        self
    }

    pub fn tls_config(mut self, config: TlsConfig) -> Self {
        self.config.tls_config = Some(config);
        self
    }

    pub fn build(self) -> HttpsServer {
        HttpsServer { config: self.config, router: self.router }
    }
}

impl Default for HttpsServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HttpsServer {
    config: HttpsServerConfig,
    router: AxumRouter,
}

impl HttpsServer {
    pub async fn serve(self) -> HttpsResult<()> {
        let addr = self.config.addr;
        let service_name = self.config.service_name.clone();
        let protocol_info = self.get_protocol_info();
        let tls_config = self.config.tls_config.clone();

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| WaeError::internal(format!("Failed to bind address: {}", e)))?;

        info!("{} {} server starting on {}", service_name, protocol_info, addr);

        match tls_config {
            Some(tls_config) => self.serve_tls(listener, &tls_config).await,
            None => self.serve_plain(listener).await,
        }
    }

    async fn serve_plain(self, listener: TcpListener) -> HttpsResult<()> {
        let app = self.router;
        axum::serve(listener, app)
            .await
            .map_err(|e| WaeError::internal(format!("Server error: {}", e)))?;
        Ok(())
    }

    async fn serve_tls(self, listener: TcpListener, tls_config: &TlsConfig) -> HttpsResult<()> {
        let tls_acceptor = tls::create_tls_acceptor_with_http2(
            &tls_config.cert_path,
            &tls_config.key_path,
            self.config.http2_config.enabled,
        )
        .map_err(|e| WaeError::internal(format!("TLS config error: {}", e)))?;

        let app = self.router;

        loop {
            let (stream, _remote_addr) = listener
                .accept()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to accept connection: {}", e)))?;

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

    fn get_protocol_info(&self) -> String {
        let tls_info = if self.config.tls_config.is_some() { "S" } else { "" };
        let version_info = match self.config.http_version {
            HttpVersion::Http1Only => "HTTP/1.1",
            HttpVersion::Http2Only => "HTTP/2",
            HttpVersion::Both => "HTTP/1.1+HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        };
        format!("{}{}", version_info, tls_info)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiErrorBody>,
    pub trace_id: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
}

impl<T: serde::Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success { StatusCode::OK } else { StatusCode::BAD_REQUEST };
        let body = serde_json::to_string(&self).unwrap_or_default();
        Response::builder()
            .status(status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}

/// 创建静态文件服务路由
///
/// 用于提供静态资源文件服务。
///
/// # 参数
///
/// * `path` - 静态文件所在的目录路径
/// * `prefix` - 路由前缀，例如 "/static"
pub fn static_files_router(path: impl AsRef<Path>, prefix: &str) -> AxumRouter {
    let serve_dir = ServeDir::new(path);
    AxumRouter::new().nest_service(prefix, serve_dir)
}

impl<T> ApiResponse<T>
where
    T: serde::Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            trace_id: None,
        }
    }

    pub fn success_with_trace(data: T, trace_id: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            trace_id: Some(trace_id.into()),
        }
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiErrorBody {
                code: code.into(),
                message: message.into(),
            }),
            trace_id: None,
        }
    }

    pub fn error_with_trace(
        code: impl Into<String>,
        message: impl Into<String>,
        trace_id: impl Into<String>,
    ) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiErrorBody {
                code: code.into(),
                message: message.into(),
            }),
            trace_id: Some(trace_id.into()),
        }
    }
}
