//! TOTP 错误类型定义

use std::fmt;

/// TOTP 错误类型
#[derive(Debug)]
pub enum TotpError {
    /// 无效的密钥
    InvalidSecret(String),

    /// 无效的验证码
    InvalidCode,

    /// 验证码已过期
    CodeExpired,

    /// 验证码格式错误
    InvalidCodeFormat { expected: usize, actual: usize },

    /// 时间步长错误
    InvalidTimeStep,

    /// Base32 解码错误
    Base32Error(String),

    /// HMAC 错误
    HmacError(String),

    /// QR 码生成错误
    QrCodeError(String),

    /// 配置错误
    ConfigurationError(String),

    /// 其他错误
    Other(String),
}

impl fmt::Display for TotpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TotpError::InvalidSecret(msg) => write!(f, "Invalid secret: {}", msg),
            TotpError::InvalidCode => write!(f, "Invalid code"),
            TotpError::CodeExpired => write!(f, "Code has expired"),
            TotpError::InvalidCodeFormat { expected, actual } => {
                write!(f, "Invalid code format: expected {} digits, got {}", expected, actual)
            }
            TotpError::InvalidTimeStep => write!(f, "Invalid time step"),
            TotpError::Base32Error(msg) => write!(f, "Base32 decode error: {}", msg),
            TotpError::HmacError(msg) => write!(f, "HMAC error: {}", msg),
            TotpError::QrCodeError(msg) => write!(f, "QR code error: {}", msg),
            TotpError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            TotpError::Other(msg) => write!(f, "TOTP error: {}", msg),
        }
    }
}

impl std::error::Error for TotpError {}

/// TOTP 结果类型
pub type TotpResult<T> = Result<T, TotpError>;
