//! 请求 ID 中间件
//!
//! 为每个请求生成唯一的追踪 ID。

use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use uuid::Uuid;

/// 请求 ID 头名称
pub const X_REQUEST_ID: &str = "x-request-id";

/// 请求 ID 中间件层
pub struct RequestIdLayer;

impl RequestIdLayer {
    /// 创建请求 ID 中间件
    pub async fn middleware(request: Request, next: Next) -> Response<Body> {
        let request_id = request
            .headers()
            .get(X_REQUEST_ID)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let mut response = next.run(request).await;
        response.headers_mut().insert(X_REQUEST_ID, request_id.parse().unwrap());
        response
    }
}
