//! HTTP 响应模块
//!
//! 提供统一的响应处理工具。

use axum::{
    body::Body,
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use serde::Serialize;

use crate::ApiResponse;

/// JSON 响应构建器
pub struct JsonResponse;

impl JsonResponse {
    /// 返回成功响应
    pub fn success<T: Serialize>(data: T) -> Response<Body> {
        ApiResponse::success(data).into_response()
    }

    /// 返回错误响应
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Response<Body> {
        ApiResponse::<()>::error(code, message).into_response()
    }

    /// 返回 404 错误
    pub fn not_found(message: impl Into<String>) -> Response<Body> {
        let body = ApiResponse::<()>::error("NOT_FOUND", message);
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap_or_default()))
            .unwrap()
    }

    /// 返回 500 错误
    pub fn internal_error(message: impl Into<String>) -> Response<Body> {
        let body = ApiResponse::<()>::error("INTERNAL_ERROR", message);
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap_or_default()))
            .unwrap()
    }
}
