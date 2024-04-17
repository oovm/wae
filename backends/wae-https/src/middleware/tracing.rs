//! 追踪中间件
//!
//! 为请求添加追踪日志。

use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use tracing::{Instrument, info, info_span};

/// 追踪中间件层
pub struct TracingLayer;

impl TracingLayer {
    /// 创建追踪中间件
    pub async fn middleware(request: Request, next: Next) -> Response<Body> {
        let method = request.method().to_string();
        let uri = request.uri().to_string();
        let span = info_span!("http_request", method = %method, uri = %uri);

        async move {
            info!("Request started");
            let response = next.run(request).await;
            info!("Request completed with status: {}", response.status());
            response
        }
        .instrument(span)
        .await
    }
}
