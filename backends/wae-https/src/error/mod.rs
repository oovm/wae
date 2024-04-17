//! HTTP 错误处理模块
//!
//! 提供统一的错误处理机制，包括：
//! - HTTP 错误类型枚举
//! - 错误响应格式化
//! - 错误扩展 trait
//! - 与 axum 框架的集成

use axum::{
    body::Body,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// HTTP 错误类型枚举
///
/// 定义了 HTTP 服务中可能出现的各种错误类型，
/// 每种错误类型对应特定的 HTTP 状态码。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpError {
    /// 错误请求 (400)
    BadRequest(String),
    /// 未授权 (401)
    Unauthorized(String),
    /// 禁止访问 (403)
    Forbidden(String),
    /// 资源未找到 (404)
    NotFound(String),
    /// 方法不允许 (405)
    MethodNotAllowed(String),
    /// 请求超时 (408)
    RequestTimeout(String),
    /// 冲突 (409)
    Conflict(String),
    /// 请求实体过大 (413)
    PayloadTooLarge(String),
    /// URI 过长 (414)
    UriTooLong(String),
    /// 不支持的媒体类型 (415)
    UnsupportedMediaType(String),
    /// 请求格式错误 (422)
    UnprocessableEntity(String),
    /// 请求过于频繁 (429)
    TooManyRequests(String),
    /// 内部服务器错误 (500)
    InternalServerError(String),
    /// 服务不可用 (503)
    ServiceUnavailable(String),
    /// 网关超时 (504)
    GatewayTimeout(String),
}

impl HttpError {
    /// 获取错误对应的 HTTP 状态码
    pub fn status_code(&self) -> StatusCode {
        match self {
            HttpError::BadRequest(_) => StatusCode::BAD_REQUEST,
            HttpError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            HttpError::Forbidden(_) => StatusCode::FORBIDDEN,
            HttpError::NotFound(_) => StatusCode::NOT_FOUND,
            HttpError::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            HttpError::RequestTimeout(_) => StatusCode::REQUEST_TIMEOUT,
            HttpError::Conflict(_) => StatusCode::CONFLICT,
            HttpError::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            HttpError::UriTooLong(_) => StatusCode::URI_TOO_LONG,
            HttpError::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            HttpError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            HttpError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            HttpError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            HttpError::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    /// 获取错误码字符串
    pub fn error_code(&self) -> &'static str {
        match self {
            HttpError::BadRequest(_) => "BAD_REQUEST",
            HttpError::Unauthorized(_) => "UNAUTHORIZED",
            HttpError::Forbidden(_) => "FORBIDDEN",
            HttpError::NotFound(_) => "NOT_FOUND",
            HttpError::MethodNotAllowed(_) => "METHOD_NOT_ALLOWED",
            HttpError::RequestTimeout(_) => "REQUEST_TIMEOUT",
            HttpError::Conflict(_) => "CONFLICT",
            HttpError::PayloadTooLarge(_) => "PAYLOAD_TOO_LARGE",
            HttpError::UriTooLong(_) => "URI_TOO_LONG",
            HttpError::UnsupportedMediaType(_) => "UNSUPPORTED_MEDIA_TYPE",
            HttpError::UnprocessableEntity(_) => "UNPROCESSABLE_ENTITY",
            HttpError::TooManyRequests(_) => "TOO_MANY_REQUESTS",
            HttpError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            HttpError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            HttpError::GatewayTimeout(_) => "GATEWAY_TIMEOUT",
        }
    }

    /// 获取错误消息
    pub fn message(&self) -> &str {
        match self {
            HttpError::BadRequest(msg) => msg,
            HttpError::Unauthorized(msg) => msg,
            HttpError::Forbidden(msg) => msg,
            HttpError::NotFound(msg) => msg,
            HttpError::MethodNotAllowed(msg) => msg,
            HttpError::RequestTimeout(msg) => msg,
            HttpError::Conflict(msg) => msg,
            HttpError::PayloadTooLarge(msg) => msg,
            HttpError::UriTooLong(msg) => msg,
            HttpError::UnsupportedMediaType(msg) => msg,
            HttpError::UnprocessableEntity(msg) => msg,
            HttpError::TooManyRequests(msg) => msg,
            HttpError::InternalServerError(msg) => msg,
            HttpError::ServiceUnavailable(msg) => msg,
            HttpError::GatewayTimeout(msg) => msg,
        }
    }

    /// 创建错误请求错误
    pub fn bad_request(msg: impl Into<String>) -> Self {
        HttpError::BadRequest(msg.into())
    }

    /// 创建未授权错误
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        HttpError::Unauthorized(msg.into())
    }

    /// 创建禁止访问错误
    pub fn forbidden(msg: impl Into<String>) -> Self {
        HttpError::Forbidden(msg.into())
    }

    /// 创建资源未找到错误
    pub fn not_found(msg: impl Into<String>) -> Self {
        HttpError::NotFound(msg.into())
    }

    /// 创建方法不允许错误
    pub fn method_not_allowed(msg: impl Into<String>) -> Self {
        HttpError::MethodNotAllowed(msg.into())
    }

    /// 创建请求超时错误
    pub fn request_timeout(msg: impl Into<String>) -> Self {
        HttpError::RequestTimeout(msg.into())
    }

    /// 创建冲突错误
    pub fn conflict(msg: impl Into<String>) -> Self {
        HttpError::Conflict(msg.into())
    }

    /// 创建请求实体过大错误
    pub fn payload_too_large(msg: impl Into<String>) -> Self {
        HttpError::PayloadTooLarge(msg.into())
    }

    /// 创建内部服务器错误
    pub fn internal(msg: impl Into<String>) -> Self {
        HttpError::InternalServerError(msg.into())
    }

    /// 创建服务不可用错误
    pub fn service_unavailable(msg: impl Into<String>) -> Self {
        HttpError::ServiceUnavailable(msg.into())
    }

    /// 创建网关超时错误
    pub fn gateway_timeout(msg: impl Into<String>) -> Self {
        HttpError::GatewayTimeout(msg.into())
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_code(), self.message())
    }
}

impl std::error::Error for HttpError {}

/// HTTP 操作结果类型别名
pub type HttpResult<T> = Result<T, HttpError>;

/// 错误响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// 是否成功
    pub success: bool,
    /// 错误码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 详细信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// 请求追踪 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl ErrorResponse {
    /// 从 HttpError 创建错误响应
    pub fn from_error(error: &HttpError) -> Self {
        Self {
            success: false,
            code: error.error_code().to_string(),
            message: error.message().to_string(),
            details: None,
            trace_id: None,
        }
    }

    /// 从 HttpError 创建错误响应（带详细信息）
    pub fn from_error_with_details(error: &HttpError, details: serde_json::Value) -> Self {
        Self {
            success: false,
            code: error.error_code().to_string(),
            message: error.message().to_string(),
            details: Some(details),
            trace_id: None,
        }
    }

    /// 设置追踪 ID
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = ErrorResponse::from_error(&self);
        let body = serde_json::to_string(&error_response).unwrap_or_else(|_| {
            r#"{"success":false,"code":"INTERNAL_ERROR","message":"Failed to serialize error"}"#.to_string()
        });

        Response::builder().status(status).header(header::CONTENT_TYPE, "application/json").body(Body::from(body)).unwrap()
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;
        let body = serde_json::to_string(&self).unwrap_or_else(|_| {
            r#"{"success":false,"code":"INTERNAL_ERROR","message":"Failed to serialize error"}"#.to_string()
        });

        Response::builder().status(status).header(header::CONTENT_TYPE, "application/json").body(Body::from(body)).unwrap()
    }
}

/// 错误扩展 trait
///
/// 为 Result 类型提供便捷的错误转换方法。
pub trait ErrorExt<T> {
    /// 将错误转换为 HTTP 错误请求错误 (400)
    fn bad_request(self) -> HttpResult<T>;

    /// 将错误转换为 HTTP 未授权错误 (401)
    fn unauthorized(self) -> HttpResult<T>;

    /// 将错误转换为 HTTP 禁止访问错误 (403)
    fn forbidden(self) -> HttpResult<T>;

    /// 将错误转换为 HTTP 资源未找到错误 (404)
    fn not_found(self) -> HttpResult<T>;

    /// 将错误转换为 HTTP 内部服务器错误 (500)
    fn internal_error(self) -> HttpResult<T>;

    /// 使用自定义错误消息转换为 HTTP 错误
    fn with_http_error(self, error: HttpError) -> HttpResult<T>;

    /// 使用错误消息转换函数
    fn map_http_error<F>(self, f: F) -> HttpResult<T>
    where
        F: FnOnce(String) -> HttpError;
}

impl<T, E: fmt::Display> ErrorExt<T> for Result<T, E> {
    fn bad_request(self) -> HttpResult<T> {
        self.map_err(|e| HttpError::BadRequest(e.to_string()))
    }

    fn unauthorized(self) -> HttpResult<T> {
        self.map_err(|e| HttpError::Unauthorized(e.to_string()))
    }

    fn forbidden(self) -> HttpResult<T> {
        self.map_err(|e| HttpError::Forbidden(e.to_string()))
    }

    fn not_found(self) -> HttpResult<T> {
        self.map_err(|e| HttpError::NotFound(e.to_string()))
    }

    fn internal_error(self) -> HttpResult<T> {
        self.map_err(|e| HttpError::InternalServerError(e.to_string()))
    }

    fn with_http_error(self, error: HttpError) -> HttpResult<T> {
        self.map_err(|_| error)
    }

    fn map_http_error<F>(self, f: F) -> HttpResult<T>
    where
        F: FnOnce(String) -> HttpError,
    {
        self.map_err(|e| f(e.to_string()))
    }
}

/// 从 wae_types::WaeError 转换为 HttpError
impl From<wae_types::WaeError> for HttpError {
    fn from(error: wae_types::WaeError) -> Self {
        use wae_types::ErrorKind;
        match error.kind {
            ErrorKind::Validation => HttpError::BadRequest(error.to_string()),
            ErrorKind::Network => HttpError::ServiceUnavailable(error.to_string()),
            ErrorKind::Storage => HttpError::InternalServerError(error.to_string()),
            ErrorKind::Internal => HttpError::InternalServerError(error.to_string()),
            ErrorKind::NotFound => HttpError::NotFound(error.to_string()),
            ErrorKind::Permission => HttpError::Forbidden(error.to_string()),
            ErrorKind::Timeout => HttpError::RequestTimeout(error.to_string()),
            ErrorKind::Config => HttpError::InternalServerError(error.to_string()),
        }
    }
}

/// 从 std::io::Error 转换为 HttpError
impl From<std::io::Error> for HttpError {
    fn from(error: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match error.kind() {
            ErrorKind::NotFound => HttpError::NotFound(error.to_string()),
            ErrorKind::PermissionDenied => HttpError::Forbidden(error.to_string()),
            ErrorKind::TimedOut => HttpError::RequestTimeout(error.to_string()),
            ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted => {
                HttpError::ServiceUnavailable(error.to_string())
            }
            _ => HttpError::InternalServerError(error.to_string()),
        }
    }
}

/// 从 serde_json::Error 转换为 HttpError
impl From<serde_json::Error> for HttpError {
    fn from(error: serde_json::Error) -> Self {
        HttpError::BadRequest(format!("JSON 解析错误: {}", error))
    }
}

/// 创建成功响应的便捷函数
pub fn success_response<T: Serialize>(data: T) -> Response {
    let body = serde_json::to_string(&serde_json::json!({
        "success": true,
        "data": data
    }))
    .unwrap_or_default();

    Response::builder().status(StatusCode::OK).header(header::CONTENT_TYPE, "application/json").body(Body::from(body)).unwrap()
}

/// 创建分页响应的便捷函数
pub fn paginated_response<T: Serialize>(items: Vec<T>, total: u64, page: u32, page_size: u32) -> Response {
    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    let body = serde_json::to_string(&serde_json::json!({
        "success": true,
        "data": {
            "items": items,
            "pagination": {
                "total": total,
                "page": page,
                "page_size": page_size,
                "total_pages": total_pages
            }
        }
    }))
    .unwrap_or_default();

    Response::builder().status(StatusCode::OK).header(header::CONTENT_TYPE, "application/json").body(Body::from(body)).unwrap()
}
