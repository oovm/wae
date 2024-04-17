//! OAuth2 错误类型定义

use std::fmt;

/// OAuth2 错误类型
#[derive(Debug)]
pub enum OAuth2Error {
    /// 无效的授权码
    InvalidAuthorizationCode,

    /// 无效的重定向 URI
    InvalidRedirectUri,

    /// 无效的客户端 ID
    InvalidClientId,

    /// 无效的客户端密钥
    InvalidClientSecret,

    /// 无效的授权范围
    InvalidScope(String),

    /// 访问令牌无效
    InvalidAccessToken,

    /// 刷新令牌无效
    InvalidRefreshToken,

    /// 令牌已过期
    TokenExpired,

    /// 授权被拒绝
    AccessDenied(String),

    /// 不支持的授权类型
    UnsupportedGrantType(String),

    /// 不支持的响应类型
    UnsupportedResponseType(String),

    /// 状态不匹配
    StateMismatch,

    /// 配置错误
    ConfigurationError(String),

    /// 网络请求错误
    RequestError(String),

    /// 解析错误
    ParseError(String),

    /// 提供者错误
    ProviderError(String),

    /// 其他错误
    Other(String),
}

impl fmt::Display for OAuth2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuth2Error::InvalidAuthorizationCode => write!(f, "Invalid authorization code"),
            OAuth2Error::InvalidRedirectUri => write!(f, "Invalid redirect URI"),
            OAuth2Error::InvalidClientId => write!(f, "Invalid client ID"),
            OAuth2Error::InvalidClientSecret => write!(f, "Invalid client secret"),
            OAuth2Error::InvalidScope(scope) => write!(f, "Invalid scope: {}", scope),
            OAuth2Error::InvalidAccessToken => write!(f, "Invalid access token"),
            OAuth2Error::InvalidRefreshToken => write!(f, "Invalid refresh token"),
            OAuth2Error::TokenExpired => write!(f, "Token has expired"),
            OAuth2Error::AccessDenied(msg) => write!(f, "Authorization denied: {}", msg),
            OAuth2Error::UnsupportedGrantType(grant_type) => {
                write!(f, "Unsupported grant type: {}", grant_type)
            }
            OAuth2Error::UnsupportedResponseType(response_type) => {
                write!(f, "Unsupported response type: {}", response_type)
            }
            OAuth2Error::StateMismatch => write!(f, "State mismatch"),
            OAuth2Error::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            OAuth2Error::RequestError(msg) => write!(f, "Request error: {}", msg),
            OAuth2Error::ParseError(msg) => write!(f, "Parse error: {}", msg),
            OAuth2Error::ProviderError(msg) => write!(f, "Provider error: {}", msg),
            OAuth2Error::Other(msg) => write!(f, "OAuth2 error: {}", msg),
        }
    }
}

impl std::error::Error for OAuth2Error {}

impl From<url::ParseError> for OAuth2Error {
    fn from(err: url::ParseError) -> Self {
        OAuth2Error::ParseError(err.to_string())
    }
}

impl From<serde_json::Error> for OAuth2Error {
    fn from(err: serde_json::Error) -> Self {
        OAuth2Error::ParseError(err.to_string())
    }
}

/// OAuth2 结果类型
pub type OAuth2Result<T> = Result<T, OAuth2Error>;
