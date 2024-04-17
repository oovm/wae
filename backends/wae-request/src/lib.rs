//! WAE Request - 纯 Tokio HTTP 客户端
//!
//! 基于 tokio + tokio-rustls 实现的 HTTP/1.1 客户端
//! 不依赖 hyper 或 reqwest

#![warn(missing_docs)]

use serde::{Serialize, de::DeserializeOwned};
use std::{collections::HashMap, fmt, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    time::timeout,
};
use tokio_rustls::{TlsConnector, rustls::pki_types::ServerName};
use tracing::{debug, error, info};
use url::Url;
use wae_types::{NetworkErrorKind, WaeError, WaeResult};

/// HTTP 客户端错误
#[derive(Debug)]
pub enum HttpError {
    /// URL 解析错误
    InvalidUrl(String),

    /// DNS 解析错误
    DnsFailed(String),

    /// 连接错误
    ConnectionFailed(String),

    /// TLS 错误
    TlsError(String),

    /// 请求超时
    Timeout,

    /// HTTP 协议错误
    ProtocolError(String),

    /// 响应解析错误
    ParseError(String),

    /// 序列化错误
    SerializationError(String),

    /// 状态码错误
    StatusError {
        /// HTTP 状态码
        status: u16,
        /// 响应体
        body: String,
    },
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            HttpError::DnsFailed(msg) => write!(f, "DNS resolution failed: {}", msg),
            HttpError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            HttpError::TlsError(msg) => write!(f, "TLS error: {}", msg),
            HttpError::Timeout => write!(f, "Request timeout"),
            HttpError::ProtocolError(msg) => write!(f, "HTTP protocol error: {}", msg),
            HttpError::ParseError(msg) => write!(f, "Response parse error: {}", msg),
            HttpError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            HttpError::StatusError { status, body } => write!(f, "HTTP {}: {}", status, body),
        }
    }
}

impl std::error::Error for HttpError {}

/// HTTP 客户端配置
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// 请求超时时间
    pub timeout: Duration,
    /// 连接超时时间
    pub connect_timeout: Duration,
    /// 用户代理
    pub user_agent: String,
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔
    pub retry_delay: Duration,
    /// 默认请求头
    pub default_headers: HashMap<String, String>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            user_agent: "wae-request/0.1.0".to_string(),
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
            default_headers: HashMap::new(),
        }
    }
}

/// HTTP 响应
#[derive(Debug)]
pub struct HttpResponse {
    /// HTTP 版本
    pub version: String,
    /// 状态码
    pub status: u16,
    /// 状态文本
    pub status_text: String,
    /// 响应头
    pub headers: HashMap<String, String>,
    /// 响应体
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// 解析 JSON 响应体
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, HttpError> {
        serde_json::from_slice(&self.body).map_err(|e| HttpError::ParseError(format!("JSON parse error: {}", e)))
    }

    /// 获取文本响应体
    pub fn text(&self) -> Result<String, HttpError> {
        String::from_utf8(self.body.clone()).map_err(|e| HttpError::ParseError(format!("UTF-8 decode error: {}", e)))
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

/// TLS 连接器 (全局共享)
static TLS_CONNECTOR: std::sync::OnceLock<Arc<TlsConnector>> = std::sync::OnceLock::new();

fn get_tls_connector() -> Arc<TlsConnector> {
    TLS_CONNECTOR
        .get_or_init(|| {
            let mut roots = tokio_rustls::rustls::RootCertStore::empty();
            roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            let config = tokio_rustls::rustls::ClientConfig::builder().with_root_certificates(roots).with_no_client_auth();
            Arc::new(TlsConnector::from(Arc::new(config)))
        })
        .clone()
}

/// HTTP 客户端
#[derive(Debug, Clone)]
pub struct HttpClient {
    config: HttpClientConfig,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new(HttpClientConfig::default())
    }
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new(config: HttpClientConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::default()
    }

    /// 获取配置
    pub fn config(&self) -> &HttpClientConfig {
        &self.config
    }

    /// 发送 GET 请求
    pub async fn get(&self, url: &str) -> Result<HttpResponse, HttpError> {
        self.request("GET", url, None, None).await
    }

    /// 发送带请求头的 GET 请求
    pub async fn get_with_headers(&self, url: &str, headers: HashMap<String, String>) -> Result<HttpResponse, HttpError> {
        self.request("GET", url, None, Some(headers)).await
    }

    /// 发送 POST JSON 请求
    pub async fn post_json<T: Serialize>(&self, url: &str, body: &T) -> Result<HttpResponse, HttpError> {
        let json_body = serde_json::to_vec(body).map_err(|e| HttpError::SerializationError(e.to_string()))?;

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        self.request("POST", url, Some(json_body), Some(headers)).await
    }

    /// 发送带请求头的 POST 请求
    pub async fn post_with_headers(
        &self,
        url: &str,
        body: Vec<u8>,
        headers: HashMap<String, String>,
    ) -> Result<HttpResponse, HttpError> {
        self.request("POST", url, Some(body), Some(headers)).await
    }

    /// 发送请求 (带重试)
    pub async fn request(
        &self,
        method: &str,
        url: &str,
        body: Option<Vec<u8>>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                let delay = self.config.retry_delay * attempt;
                debug!("Retry attempt {} after {:?}", attempt, delay);
                tokio::time::sleep(delay).await;
            }

            match self.request_once(method, url, body.clone(), headers.clone()).await {
                Ok(response) => {
                    if response.is_success() {
                        info!("Request succeeded on attempt {}", attempt);
                        return Ok(response);
                    }

                    if Self::is_retryable_status(response.status) && attempt < self.config.max_retries {
                        last_error = Some(HttpError::StatusError {
                            status: response.status,
                            body: String::from_utf8_lossy(&response.body).to_string(),
                        });
                        continue;
                    }

                    return Err(HttpError::StatusError {
                        status: response.status,
                        body: String::from_utf8_lossy(&response.body).to_string(),
                    });
                }
                Err(e) => {
                    error!("Request error on attempt {}: {}", attempt, e);
                    if Self::is_retryable_error(&e) && attempt < self.config.max_retries {
                        last_error = Some(e);
                        continue;
                    }
                    return Err(e);
                }
            }
        }

        Err(last_error.unwrap_or(HttpError::Timeout))
    }

    /// 发送单次请求
    async fn request_once(
        &self,
        method: &str,
        url_str: &str,
        body: Option<Vec<u8>>,
        extra_headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError> {
        let url = Url::parse(url_str).map_err(|e| HttpError::InvalidUrl(e.to_string()))?;

        let host = url.host_str().ok_or_else(|| HttpError::InvalidUrl("Missing host".into()))?;
        let port = url.port().unwrap_or(if url.scheme() == "https" { 443 } else { 80 });
        let path = url.path();
        let query = url.query().map(|q| format!("?{}", q)).unwrap_or_default();
        let uri = format!("{}{}", path, query);

        let is_https = url.scheme() == "https";

        let connect_result =
            timeout(self.config.connect_timeout, TcpStream::connect((host, port))).await.map_err(|_| HttpError::Timeout)?;

        let tcp_stream = connect_result.map_err(|e| HttpError::ConnectionFailed(format!("TCP connect failed: {}", e)))?;

        tcp_stream.set_nodelay(true).ok();

        let response = if is_https {
            let connector = get_tls_connector();
            let server_name = ServerName::try_from(host.to_string())
                .map_err(|e| HttpError::TlsError(format!("Invalid server name: {}", e)))?;

            let tls_stream = connector
                .connect(server_name, tcp_stream)
                .await
                .map_err(|e| HttpError::TlsError(format!("TLS handshake failed: {}", e)))?;

            let (reader, writer) = tokio::io::split(tls_stream);
            self.send_http_request(reader, writer, method, host, &uri, body, extra_headers).await?
        }
        else {
            let (reader, writer) = tcp_stream.into_split();
            self.send_http_request(reader, writer, method, host, &uri, body, extra_headers).await?
        };

        Ok(response)
    }

    /// 发送 HTTP 请求并读取响应
    #[allow(clippy::too_many_arguments)]
    async fn send_http_request<R, W>(
        &self,
        reader: R,
        mut writer: W,
        method: &str,
        host: &str,
        uri: &str,
        body: Option<Vec<u8>>,
        extra_headers: Option<HashMap<String, String>>,
    ) -> Result<HttpResponse, HttpError>
    where
        R: AsyncReadExt + Unpin,
        W: AsyncWriteExt + Unpin,
    {
        let body_len = body.as_ref().map(|b| b.len()).unwrap_or(0);

        let mut request =
            format!("{} {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\n", method, uri, host, self.config.user_agent);

        if body_len > 0 {
            request.push_str(&format!("Content-Length: {}\r\n", body_len));
        }

        for (key, value) in &self.config.default_headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }

        if let Some(headers) = extra_headers {
            for (key, value) in headers {
                request.push_str(&format!("{}: {}\r\n", key, value));
            }
        }

        request.push_str("Connection: close\r\n\r\n");

        let mut request_bytes = request.into_bytes();
        if let Some(b) = body {
            request_bytes.extend(b);
        }

        timeout(self.config.timeout, async {
            writer
                .write_all(&request_bytes)
                .await
                .map_err(|e| HttpError::ConnectionFailed(format!("Write request failed: {}", e)))?;
            writer.flush().await.map_err(|e| HttpError::ConnectionFailed(format!("Flush failed: {}", e)))?;
            Ok::<_, HttpError>(())
        })
        .await
        .map_err(|_| HttpError::Timeout)??;

        let response = timeout(self.config.timeout, self.read_response(reader)).await.map_err(|_| HttpError::Timeout)??;

        Ok(response)
    }

    /// 读取 HTTP 响应
    async fn read_response<R: AsyncReadExt + Unpin>(&self, reader: R) -> Result<HttpResponse, HttpError> {
        let mut buf_reader = BufReader::new(reader);
        let mut status_line = String::new();

        buf_reader
            .read_line(&mut status_line)
            .await
            .map_err(|e| HttpError::ProtocolError(format!("Read status line failed: {}", e)))?;

        let status_parts: Vec<&str> = status_line.trim().splitn(3, ' ').collect();
        if status_parts.len() < 2 {
            return Err(HttpError::ProtocolError("Invalid status line".into()));
        }

        let version = status_parts[0].to_string();
        let status: u16 = status_parts[1].parse().map_err(|_| HttpError::ProtocolError("Invalid status code".into()))?;
        let status_text = status_parts.get(2).unwrap_or(&"").to_string();

        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            buf_reader
                .read_line(&mut line)
                .await
                .map_err(|e| HttpError::ProtocolError(format!("Read header failed: {}", e)))?;

            if line == "\r\n" || line.is_empty() {
                break;
            }

            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let content_length: Option<usize> = headers.get("content-length").and_then(|v| v.parse().ok());

        let mut body = Vec::new();

        if let Some(len) = content_length {
            body.resize(len, 0);
            buf_reader.read_exact(&mut body).await.map_err(|e| HttpError::ProtocolError(format!("Read body failed: {}", e)))?;
        }
        else {
            buf_reader
                .read_to_end(&mut body)
                .await
                .map_err(|e| HttpError::ProtocolError(format!("Read body failed: {}", e)))?;
        }

        Ok(HttpResponse { version, status, status_text, headers, body })
    }

    /// 判断状态码是否可重试
    fn is_retryable_status(status: u16) -> bool {
        matches!(status, 408 | 429 | 500 | 502 | 503 | 504)
    }

    /// 判断错误是否可重试
    fn is_retryable_error(error: &HttpError) -> bool {
        matches!(error, HttpError::Timeout | HttpError::ConnectionFailed(_) | HttpError::DnsFailed(_))
    }
}

/// 请求构建器
pub struct RequestBuilder {
    client: HttpClient,
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    /// 创建新的请求构建器
    pub fn new(client: HttpClient, method: &str, url: &str) -> Self {
        Self { client, method: method.to_string(), url: url.to_string(), headers: HashMap::new(), body: None }
    }

    /// 添加请求头
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// 添加 Bearer Token
    pub fn bearer_auth(self, token: impl Into<String>) -> Self {
        self.header("Authorization", format!("Bearer {}", token.into()))
    }

    /// 设置 JSON 请求体
    pub fn json<T: Serialize>(mut self, body: &T) -> Self {
        if let Ok(json) = serde_json::to_vec(body) {
            self.headers.insert("Content-Type".into(), "application/json".into());
            self.body = Some(json);
        }
        self
    }

    /// 设置原始请求体
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// 发送请求
    pub async fn send(self) -> Result<HttpResponse, HttpError> {
        self.client.request(&self.method, &self.url, self.body, Some(self.headers)).await
    }

    /// 发送请求并解析 JSON
    pub async fn send_json<T: DeserializeOwned>(self) -> Result<T, HttpError> {
        let response = self.send().await?;
        response.json()
    }
}

/// 便捷函数：创建 GET 请求
pub fn get(url: &str) -> RequestBuilder {
    RequestBuilder::new(HttpClient::default(), "GET", url)
}

/// 便捷函数：创建 POST 请求
pub fn post(url: &str) -> RequestBuilder {
    RequestBuilder::new(HttpClient::default(), "POST", url)
}

/// 将 HttpError 转换为 WaeError
impl From<HttpError> for WaeError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::InvalidUrl(msg) => WaeError::network(NetworkErrorKind::ConnectionFailed).with_param("message", msg),
            HttpError::Timeout => WaeError::network(NetworkErrorKind::Timeout),
            HttpError::ConnectionFailed(msg) => {
                WaeError::network(NetworkErrorKind::ConnectionFailed).with_param("message", msg)
            }
            HttpError::DnsFailed(msg) => WaeError::network(NetworkErrorKind::DnsFailed).with_param("detail", msg),
            HttpError::TlsError(msg) => WaeError::network(NetworkErrorKind::TlsError).with_param("message", msg),
            HttpError::StatusError { status, body } => WaeError::network(NetworkErrorKind::ProtocolError)
                .with_param("status", status.to_string())
                .with_param("body", body),
            HttpError::ProtocolError(msg) => WaeError::network(NetworkErrorKind::ProtocolError).with_param("message", msg),
            HttpError::ParseError(msg) => WaeError::internal(format!("Response parse error: {}", msg)),
            HttpError::SerializationError(msg) => WaeError::internal(format!("Serialization error: {}", msg)),
        }
    }
}

/// 兼容旧 API 的 RequestClient
#[derive(Debug, Clone)]
pub struct RequestClient {
    client: HttpClient,
    config: RequestConfig,
}

/// 兼容旧 API 的配置
#[derive(Debug, Clone)]
pub struct RequestConfig {
    /// 请求超时时间（秒）
    pub timeout_secs: u64,
    /// 连接超时时间（秒）
    pub connect_timeout_secs: u64,
    /// 最大重试次数
    pub max_retries: u32,
    /// 重试间隔（毫秒）
    pub retry_delay_ms: u64,
    /// 用户代理
    pub user_agent: String,
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            connect_timeout_secs: 10,
            max_retries: 3,
            retry_delay_ms: 1000,
            user_agent: "wae-request/0.1.0".to_string(),
        }
    }
}

impl RequestClient {
    /// 创建新的 HTTP 客户端
    pub fn new(config: RequestConfig) -> WaeResult<Self> {
        let http_config = HttpClientConfig {
            timeout: Duration::from_secs(config.timeout_secs),
            connect_timeout: Duration::from_secs(config.connect_timeout_secs),
            user_agent: config.user_agent.clone(),
            max_retries: config.max_retries,
            retry_delay: Duration::from_millis(config.retry_delay_ms),
            default_headers: HashMap::new(),
        };
        Ok(Self { client: HttpClient::new(http_config), config })
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> WaeResult<Self> {
        Self::new(RequestConfig::default())
    }

    /// 获取配置
    pub fn config(&self) -> &RequestConfig {
        &self.config
    }

    /// 发送 GET 请求并解析 JSON
    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> WaeResult<T> {
        let response = self.client.get(url).await.map_err(WaeError::from)?;
        response.json().map_err(WaeError::from)
    }

    /// 发送 GET 请求返回原始响应
    pub async fn get_raw(&self, url: &str) -> WaeResult<HttpResponse> {
        self.client.get(url).await.map_err(WaeError::from)
    }

    /// 发送 POST JSON 请求并解析响应
    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, url: &str, body: &B) -> WaeResult<T> {
        let response = self.client.post_json(url, body).await.map_err(WaeError::from)?;
        response.json().map_err(WaeError::from)
    }

    /// 发送 POST JSON 请求返回原始响应
    pub async fn post_raw<B: Serialize>(&self, url: &str, body: &B) -> WaeResult<HttpResponse> {
        self.client.post_json(url, body).await.map_err(WaeError::from)
    }

    /// 创建请求构建器
    pub fn builder(&self) -> RequestBuilder {
        RequestBuilder::new(self.client.clone(), "GET", "")
    }
}
