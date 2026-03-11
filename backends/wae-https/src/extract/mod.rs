#![doc = include_str!("readme.md")]

use http::{Method, Uri, Version, header::HeaderMap};
use std::fmt;

/// Extractor 错误类型
///
/// 统一封装所有提取器可能产生的错误，便于错误处理和响应。
#[derive(Debug)]
pub enum ExtractorError {
    /// 路径参数提取错误
    PathRejection(String),

    /// 查询参数提取错误
    QueryRejection(String),

    /// JSON 提取错误
    JsonRejection(String),

    /// 表单数据提取错误
    FormRejection(String),

    /// 扩展数据提取错误
    ExtensionRejection(String),

    /// Host 提取错误
    HostRejection(String),

    /// TypedHeader 提取错误
    TypedHeaderRejection(String),

    /// Cookie 提取错误
    CookieRejection(String),

    /// Multipart 提取错误
    MultipartRejection(String),

    /// WebSocketUpgrade 提取错误
    WebSocketUpgradeRejection(String),

    /// Bytes 提取错误
    BytesRejection(String),

    /// Stream 提取错误
    StreamRejection(String),

    /// 自定义错误消息
    Custom(String),
}

impl fmt::Display for ExtractorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractorError::PathRejection(e) => write!(f, "Path extraction failed: {}", e),
            ExtractorError::QueryRejection(e) => write!(f, "Query extraction failed: {}", e),
            ExtractorError::JsonRejection(e) => write!(f, "Json extraction failed: {}", e),
            ExtractorError::FormRejection(e) => write!(f, "Form extraction failed: {}", e),
            ExtractorError::ExtensionRejection(e) => write!(f, "Extension extraction failed: {}", e),
            ExtractorError::HostRejection(e) => write!(f, "Host extraction failed: {}", e),
            ExtractorError::TypedHeaderRejection(e) => write!(f, "TypedHeader extraction failed: {}", e),
            ExtractorError::CookieRejection(e) => write!(f, "Cookie extraction failed: {}", e),
            ExtractorError::MultipartRejection(e) => write!(f, "Multipart extraction failed: {}", e),
            ExtractorError::WebSocketUpgradeRejection(e) => write!(f, "WebSocketUpgrade extraction failed: {}", e),
            ExtractorError::BytesRejection(e) => write!(f, "Bytes extraction failed: {}", e),
            ExtractorError::StreamRejection(e) => write!(f, "Stream extraction failed: {}", e),
            ExtractorError::Custom(msg) => write!(f, "Extractor error: {}", msg),
        }
    }
}

impl std::error::Error for ExtractorError {}

/// 请求头提取器
///
/// 用于从请求中提取指定名称的请求头值。
///
/// # 示例
///
/// ```ignore
/// use wae_https::extract::Header;
///
/// async fn handler(Header(content_type): Header<String>) -> String {
///     content_type
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Header<T>(pub T);

/// HTTP 方法提取器
///
/// 获取当前请求的 HTTP 方法（GET、POST、PUT、DELETE 等）。
pub type HttpMethod = Method;

/// 请求 URI 提取器
///
/// 获取当前请求的完整 URI。
pub type RequestUri = Uri;

/// HTTP 版本提取器
///
/// 获取当前请求的 HTTP 版本（HTTP/1.0、HTTP/1.1、HTTP/2.0 等）。
pub type HttpVersion = Version;

/// 请求头映射提取器
///
/// 获取所有请求头的键值对映射。
pub type Headers = HeaderMap;

/// 扩展数据提取器
#[derive(Debug, Clone)]
pub struct Extension<T>(pub T);

/// 表单数据提取器
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Form<T>(pub T);

/// JSON 提取器
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Json<T>(pub T);

/// 多部分表单数据提取器
#[derive(Debug, Clone)]
pub struct Multipart;

/// 原始 URI 提取器
pub type OriginalUri = http::Uri;

/// 路径参数提取器
#[derive(Debug, Clone)]
pub struct Path<T>(pub T);

/// 查询参数提取器
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Query<T>(pub T);

/// 状态提取器
#[derive(Debug, Clone)]
pub struct State<T>(pub T);

/// WebSocket 升级提取器
#[derive(Debug, Clone)]
pub struct WebSocketUpgrade;

pub use bytes::Bytes;

/// 流式请求体提取器
///
/// 用于从请求中提取流式数据。
pub type Stream = crate::Body;
