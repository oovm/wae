//! SAML 错误类型定义

use std::fmt;

/// SAML 错误类型
#[derive(Debug)]
pub enum SamlError {
    /// 无效的 SAML 请求
    InvalidRequest(String),

    /// 无效的 SAML 响应
    InvalidResponse(String),

    /// 无效的断言
    InvalidAssertion(String),

    /// 签名验证失败
    SignatureVerificationFailed(String),

    /// 缺少签名
    MissingSignature,

    /// 证书错误
    CertificateError(String),

    /// XML 解析错误
    XmlParsingError(String),

    /// Base64 解码错误
    Base64DecodeError(String),

    /// 压缩/解压错误
    CompressionError(String),

    /// 时间验证失败
    TimeValidationError(String),

    /// 断言已过期
    AssertionExpired,

    /// 断言尚未生效
    AssertionNotYetValid,

    /// 受众验证失败
    AudienceValidationFailed { expected: String, actual: String },

    /// 发行人验证失败
    IssuerValidationFailed { expected: String, actual: String },

    /// 目标验证失败
    DestinationValidationFailed,

    /// 重放攻击检测
    ReplayAttackDetected,

    /// 配置错误
    ConfigurationError(String),

    /// 元数据错误
    MetadataError(String),

    /// 绑定不支持
    UnsupportedBinding(String),

    /// 名称 ID 格式不支持
    UnsupportedNameIdFormat(String),

    /// 其他错误
    Other(String),
}

impl fmt::Display for SamlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SamlError::InvalidRequest(msg) => write!(f, "Invalid SAML request: {}", msg),
            SamlError::InvalidResponse(msg) => write!(f, "Invalid SAML response: {}", msg),
            SamlError::InvalidAssertion(msg) => write!(f, "Invalid assertion: {}", msg),
            SamlError::SignatureVerificationFailed(msg) => {
                write!(f, "Signature verification failed: {}", msg)
            }
            SamlError::MissingSignature => write!(f, "Missing signature"),
            SamlError::CertificateError(msg) => write!(f, "Certificate error: {}", msg),
            SamlError::XmlParsingError(msg) => write!(f, "XML parsing error: {}", msg),
            SamlError::Base64DecodeError(msg) => write!(f, "Base64 decode error: {}", msg),
            SamlError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            SamlError::TimeValidationError(msg) => write!(f, "Time validation failed: {}", msg),
            SamlError::AssertionExpired => write!(f, "Assertion has expired"),
            SamlError::AssertionNotYetValid => write!(f, "Assertion is not yet valid"),
            SamlError::AudienceValidationFailed { expected, actual } => {
                write!(f, "Audience validation failed: expected {}, got {}", expected, actual)
            }
            SamlError::IssuerValidationFailed { expected, actual } => {
                write!(f, "Issuer validation failed: expected {}, got {}", expected, actual)
            }
            SamlError::DestinationValidationFailed => write!(f, "Destination validation failed"),
            SamlError::ReplayAttackDetected => write!(f, "Replay attack detected"),
            SamlError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            SamlError::MetadataError(msg) => write!(f, "Metadata error: {}", msg),
            SamlError::UnsupportedBinding(binding) => write!(f, "Unsupported binding: {}", binding),
            SamlError::UnsupportedNameIdFormat(format) => {
                write!(f, "Unsupported name ID format: {}", format)
            }
            SamlError::Other(msg) => write!(f, "SAML error: {}", msg),
        }
    }
}

impl std::error::Error for SamlError {}

/// SAML 结果类型
pub type SamlResult<T> = Result<T, SamlError>;
