//! 弹性容错错误类型定义

use std::{fmt, time::Duration};

/// 弹性容错错误类型
#[derive(Debug)]
pub enum ResilienceError {
    /// 熔断器处于开启状态
    CircuitBreakerOpen,

    /// 限流器拒绝请求
    RateLimitExceeded,

    /// 重试次数耗尽
    MaxRetriesExceeded(String),

    /// 操作超时
    Timeout(Duration),

    /// 舱壁已满
    BulkheadFull(usize),

    /// 舱壁等待超时
    BulkheadWaitTimeout,

    /// 内部错误
    Internal(String),
}

impl fmt::Display for ResilienceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResilienceError::CircuitBreakerOpen => write!(f, "Circuit breaker is open"),
            ResilienceError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            ResilienceError::MaxRetriesExceeded(msg) => write!(f, "Max retries exceeded: {}", msg),
            ResilienceError::Timeout(duration) => write!(f, "Operation timeout after {:?}", duration),
            ResilienceError::BulkheadFull(max) => write!(f, "Bulkhead is full, max concurrent calls: {}", max),
            ResilienceError::BulkheadWaitTimeout => write!(f, "Bulkhead wait timeout"),
            ResilienceError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ResilienceError {}

/// 弹性容错操作结果类型
pub type ResilienceResult<T> = Result<T, ResilienceError>;
