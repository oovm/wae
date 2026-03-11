//! Prometheus 指标导出端点
//!
//! 提供 /metrics HTTP 端点，用于导出 Prometheus 格式的指标数据。

use crate::metrics::MetricsRegistry;
use http::{Response, StatusCode, header};
use http_body_util::Full;
use hyper::body::Bytes;
use prometheus::{Encoder, TextEncoder};
use std::sync::Arc;

/// Metrics 端点处理函数
///
/// 导出 Prometheus 格式的指标数据。
///
/// # Arguments
///
/// * `registry` - 指标注册器
pub async fn metrics_handler(registry: Arc<MetricsRegistry>) -> Response<Full<Bytes>> {
    let encoder = TextEncoder::new();
    let metric_families = registry.registry().gather();
    let mut buffer = Vec::new();

    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, encoder.format_type())
            .body(Full::new(Bytes::from(buffer)))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Full::new(Bytes::from("Failed to encode metrics")))
            .unwrap(),
    }
}
