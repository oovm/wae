//! JWT 错误类型定义

use std::fmt;

/// JWT 错误类型
#[derive(Debug)]
pub enum JwtError {
    /// 无效的令牌
    InvalidToken(String),

    /// 令牌已过期
    TokenExpired,

    /// 令牌尚未生效
    TokenNotValidYet,

    /// 无效的签名
    InvalidSignature,

    /// 无效的算法
    InvalidAlgorithm,

    /// 缺少必要的声明
    MissingClaim(String),

    /// 无效的声明
    InvalidClaim(String),

    /// 签发者验证失败
    InvalidIssuer { expected: String, actual: String },

    /// 受众验证失败
    InvalidAudience,

    /// 密钥错误
    KeyError(String),

    /// 编码错误
    EncodingError(String),

    /// 解码错误
    DecodingError(String),

    /// 其他错误
    Other(String),
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            JwtError::TokenExpired => write!(f, "Token has expired"),
            JwtError::TokenNotValidYet => write!(f, "Token is not yet valid"),
            JwtError::InvalidSignature => write!(f, "Invalid signature"),
            JwtError::InvalidAlgorithm => write!(f, "Invalid algorithm"),
            JwtError::MissingClaim(claim) => write!(f, "Missing required claim: {}", claim),
            JwtError::InvalidClaim(claim) => write!(f, "Invalid claim: {}", claim),
            JwtError::InvalidIssuer { expected, actual } => {
                write!(f, "Invalid issuer: expected {}, got {}", expected, actual)
            }
            JwtError::InvalidAudience => write!(f, "Invalid audience"),
            JwtError::KeyError(msg) => write!(f, "Key error: {}", msg),
            JwtError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            JwtError::DecodingError(msg) => write!(f, "Decoding error: {}", msg),
            JwtError::Other(msg) => write!(f, "JWT error: {}", msg),
        }
    }
}

impl std::error::Error for JwtError {}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;

        match err.kind() {
            ErrorKind::InvalidToken => JwtError::InvalidToken("malformed token".into()),
            ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            ErrorKind::ImmatureSignature => JwtError::TokenNotValidYet,
            ErrorKind::InvalidAlgorithm => JwtError::InvalidAlgorithm,
            ErrorKind::MissingRequiredClaim(claim) => JwtError::MissingClaim(claim.clone()),
            ErrorKind::InvalidIssuer => JwtError::InvalidClaim("issuer".into()),
            ErrorKind::InvalidAudience => JwtError::InvalidAudience,
            ErrorKind::InvalidSubject => JwtError::InvalidClaim("subject".into()),
            _ => JwtError::Other(err.to_string()),
        }
    }
}

/// JWT 结果类型
pub type JwtResult<T> = Result<T, JwtError>;
