use std::sync::Arc;
use wae_observability::{health::*, *};

#[cfg(feature = "metrics")]
use wae_observability::metrics::*;

#[cfg(feature = "profiling")]
use wae_observability::profiling::*;

#[cfg(feature = "json-log")]
use wae_observability::logging::*;

#[cfg(feature = "json-log")]
use tracing::{Level, error, info, warn};

/// 测试健康检查器
struct TestHealthChecker {
    name: String,
    status: HealthStatus,
}

impl TestHealthChecker {
    fn new(name: String, status: HealthStatus) -> Self {
        Self { name, status }
    }
}

#[async_trait::async_trait]
impl HealthChecker for TestHealthChecker {
    async fn check(&self) -> HealthCheck {
        HealthCheck::new(self.name.clone(), self.status)
    }
}

#[tokio::test]
async fn test_health_registry() {
    let mut registry = HealthRegistry::new();

    let liveness_checker = Arc::new(TestHealthChecker::new("liveness-test".to_string(), HealthStatus::Passing));
    let readiness_checker = Arc::new(TestHealthChecker::new("readiness-test".to_string(), HealthStatus::Passing));

    registry.add_checker("liveness-test".to_string(), liveness_checker, CheckCategory::Liveness);
    registry.add_checker("readiness-test".to_string(), readiness_checker, CheckCategory::Readiness);

    let liveness_report = registry.check_liveness().await;
    assert_eq!(liveness_report.overall, HealthStatus::Passing);
    assert_eq!(liveness_report.checks.len(), 1);

    let readiness_report = registry.check_readiness().await;
    assert_eq!(readiness_report.overall, HealthStatus::Passing);
    assert_eq!(readiness_report.checks.len(), 1);
}

#[tokio::test]
async fn test_health_handlers() {
    let mut registry = HealthRegistry::new();

    let liveness_checker = Arc::new(TestHealthChecker::new("liveness-test".to_string(), HealthStatus::Passing));
    registry.add_checker("liveness-test".to_string(), liveness_checker, CheckCategory::Liveness);

    let live_response = live_handler(Arc::new(registry.clone())).await;
    assert_eq!(live_response.status(), http::StatusCode::OK);

    let ready_response = ready_handler(Arc::new(registry)).await;
    assert_eq!(ready_response.status(), http::StatusCode::OK);
}

#[cfg(feature = "metrics")]
#[tokio::test]
async fn test_metrics_registry() {
    let registry = MetricsRegistry::new();

    registry.record_http_request("GET", "/test", "2xx", 0.1);
    registry.record_http_request("POST", "/test", "4xx", 0.2);

    let metric_families = registry.registry().gather();
    assert!(!metric_families.is_empty());
}

#[cfg(feature = "metrics")]
#[tokio::test]
async fn test_metrics_handler() {
    let registry = Arc::new(MetricsRegistry::new());
    let response = metrics_handler(registry).await;
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[cfg(feature = "profiling")]
#[test]
fn test_console_config() {
    let config = ConsoleConfig::new();
    assert_eq!(config.bind_addr, ([127, 0, 0, 1], 6669).into());
}

#[cfg(feature = "profiling")]
#[test]
fn test_console_config_with_addr() {
    let addr = ([127, 0, 0, 1], 9999).into();
    let config = ConsoleConfig::new().with_bind_addr(addr);
    assert_eq!(config.bind_addr, addr);
}

#[cfg(feature = "profiling")]
#[tokio::test]
async fn test_profiling_info_handler() {
    let response = profiling_info_handler("127.0.0.1:6669".to_string()).await;
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[cfg(feature = "json-log")]
#[test]
fn test_json_formatter_config_default() {
    let config = JsonFormatterConfig::default();
    assert!(config.include_timestamp);
    assert!(config.include_level);
    assert!(config.include_message);
    assert!(config.include_trace_id);
    assert!(config.include_span_id);
    assert!(config.include_target);
    assert!(config.include_file);
    assert!(config.include_line);
}

#[cfg(feature = "json-log")]
#[test]
fn test_json_formatter_builder() {
    let formatter = JsonFormatter::new()
        .include_timestamp(false)
        .include_level(false)
        .include_message(true)
        .include_trace_id(true)
        .include_span_id(true)
        .include_target(false)
        .include_file(false)
        .include_line(false);

    assert_eq!(formatter.config.include_timestamp, false);
    assert_eq!(formatter.config.include_level, false);
    assert_eq!(formatter.config.include_message, true);
}

#[cfg(feature = "json-log")]
#[test]
fn test_json_logger_config_default() {
    let config = JsonLoggerConfig::default();
    assert_eq!(config.level, Level::INFO);
    assert_eq!(config.output, LogOutput::Stdout);
}

#[cfg(feature = "json-log")]
#[test]
fn test_json_logger_config_builder() {
    let config = JsonLoggerConfig::new().with_level(Level::DEBUG).with_output(LogOutput::Stderr);

    assert_eq!(config.level, Level::DEBUG);
    assert_eq!(config.output, LogOutput::Stderr);
}

#[cfg(feature = "json-log")]
#[test]
fn test_log_output_default() {
    let output = LogOutput::default();
    assert_eq!(output, LogOutput::Stdout);
}

#[cfg(feature = "otlp")]
#[test]
fn test_otlp_config() {
    let config = OtlpConfig::new();
    assert_eq!(config.service_name, "wae-service");
    assert_eq!(config.service_version, "0.1.0");
    assert_eq!(config.endpoint, "http://localhost:4318");
}

#[cfg(feature = "otlp")]
#[test]
fn test_otlp_config_builder() {
    let config = OtlpConfig::new()
        .with_service_name("test-service".to_string())
        .with_service_version("1.0.0".to_string())
        .with_endpoint("http://localhost:4317".to_string());
    assert_eq!(config.service_name, "test-service");
    assert_eq!(config.service_version, "1.0.0");
    assert_eq!(config.endpoint, "http://localhost:4317");
}

#[cfg(feature = "otlp")]
#[test]
fn test_trace_context_extraction() {
    use http::HeaderMap;

    let mut headers = HeaderMap::new();
    let context = extract_context_from_headers(&headers);
    assert!(context.has_active_span() == false);
}

#[cfg(feature = "otlp")]
#[test]
fn test_trace_context_injection() {
    use http::HeaderMap;
    use opentelemetry::Context;

    let mut headers = HeaderMap::new();
    let context = Context::current();
    inject_context_to_headers(&context, &mut headers);
    assert!(headers.is_empty());
}

#[cfg(feature = "otlp")]
#[test]
fn test_open_telemetry_layer() {
    let layer = OpenTelemetryLayer::new();
    assert!(true, "OpenTelemetryLayer created successfully");
}

fn main() {}
