//! Trace Context 提取模块
//!
//! 提供从 HTTP 头提取和注入 traceparent 和 tracestate 的功能。

use http::{HeaderMap, HeaderName, HeaderValue};
use opentelemetry::{
    Context,
    propagation::{Extractor, Injector},
};
use opentelemetry_sdk::propagation::TraceContextPropagator;

/// HTTP 头提取器
pub struct HeaderExtractor<'a> {
    headers: &'a HeaderMap,
}

impl<'a> HeaderExtractor<'a> {
    /// 创建新的 HeaderExtractor
    pub fn new(headers: &'a HeaderMap) -> Self {
        Self { headers }
    }
}

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers.keys().map(|k| k.as_str()).collect()
    }
}

/// HTTP 头注入器
pub struct HeaderInjector<'a> {
    headers: &'a mut HeaderMap,
}

impl<'a> HeaderInjector<'a> {
    /// 创建新的 HeaderInjector
    pub fn new(headers: &'a mut HeaderMap) -> Self {
        Self { headers }
    }
}

impl<'a> Injector for HeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        if let Ok(name) = HeaderName::from_bytes(key.as_bytes()) {
            if let Ok(val) = HeaderValue::from_str(&value) {
                self.headers.insert(name, val);
            }
        }
    }
}

/// 从 HTTP 头提取 Trace Context
pub fn extract_context_from_headers(headers: &HeaderMap) -> Context {
    let extractor = HeaderExtractor::new(headers);
    opentelemetry::global::get_text_map_propagator(|propagator| propagator.extract(&extractor))
}

/// 将 Trace Context 注入 HTTP 头
pub fn inject_context_to_headers(context: &Context, headers: &mut HeaderMap) {
    let mut injector = HeaderInjector::new(headers);
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(context, &mut injector);
    });
}

/// 设置全局 Text Map 传播器
pub fn set_global_text_map_propagator() {
    let propagator = TraceContextPropagator::new();
    opentelemetry::global::set_text_map_propagator(propagator);
}
