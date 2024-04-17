#![doc = include_str!("readme.md")]

use axum::{
    body::Body,
    extract::rejection::*,
    http::{Method, Uri, Version, header::HeaderMap},
};
use std::fmt;

pub use axum::extract::{Extension, Form, Path, Query, State};

pub use axum::Json;

/// Extractor 错误类型
///
/// 统一封装所有提取器可能产生的错误，便于错误处理和响应。
#[derive(Debug)]
pub enum ExtractorError {
    /// 路径参数提取错误
    PathRejection(PathRejection),

    /// 查询参数提取错误
    QueryRejection(QueryRejection),

    /// JSON 提取错误
    JsonRejection(JsonRejection),

    /// 表单数据提取错误
    FormRejection(FormRejection),

    /// 扩展数据提取错误
    ExtensionRejection(ExtensionRejection),

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
            ExtractorError::Custom(msg) => write!(f, "Extractor error: {}", msg),
        }
    }
}

impl std::error::Error for ExtractorError {}

impl From<PathRejection> for ExtractorError {
    fn from(e: PathRejection) -> Self {
        ExtractorError::PathRejection(e)
    }
}

impl From<QueryRejection> for ExtractorError {
    fn from(e: QueryRejection) -> Self {
        ExtractorError::QueryRejection(e)
    }
}

impl From<JsonRejection> for ExtractorError {
    fn from(e: JsonRejection) -> Self {
        ExtractorError::JsonRejection(e)
    }
}

impl From<FormRejection> for ExtractorError {
    fn from(e: FormRejection) -> Self {
        ExtractorError::FormRejection(e)
    }
}

impl From<ExtensionRejection> for ExtractorError {
    fn from(e: ExtensionRejection) -> Self {
        ExtractorError::ExtensionRejection(e)
    }
}

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

/// 原始请求体提取器
///
/// 直接获取请求体的原始字节流。
pub type RawBody = Body;

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
