//! 错误类型定义模块

use std::fmt;

/// 测试错误类型
#[derive(Debug)]
pub enum TestingError {
    /// Mock 错误
    MockError(String),

    /// 断言失败
    AssertionFailed(String),

    /// 数据生成错误
    FixtureError(String),

    /// 环境设置错误
    EnvironmentError(String),

    /// 超时错误
    Timeout(String),

    /// 序列化错误
    SerializationError(String),
}

impl fmt::Display for TestingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestingError::MockError(msg) => write!(f, "Mock error: {}", msg),
            TestingError::AssertionFailed(msg) => write!(f, "Assertion failed: {}", msg),
            TestingError::FixtureError(msg) => write!(f, "Fixture generation failed: {}", msg),
            TestingError::EnvironmentError(msg) => write!(f, "Test environment setup failed: {}", msg),
            TestingError::Timeout(msg) => write!(f, "Operation timeout: {}", msg),
            TestingError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for TestingError {}

/// 测试操作结果类型
pub type TestingResult<T> = Result<T, TestingError>;
