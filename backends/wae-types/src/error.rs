//! 错误类型定义

use serde::{Deserialize, Serialize};
use std::fmt;

/// 错误类型标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorKind {
    /// 验证错误
    Validation,
    /// 网络错误
    Network,
    /// 存储错误
    Storage,
    /// 内部错误
    Internal,
    /// 未找到
    NotFound,
    /// 权限错误
    Permission,
    /// 超时错误
    Timeout,
    /// 配置错误
    Config,
}

/// 验证错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationErrorKind {
    /// 无效格式
    InvalidFormat {
        /// 字段名
        field: String,
        /// 期望值
        expected: String,
    },
    /// 值超出范围
    OutOfRange {
        /// 字段名
        field: String,
        /// 最小值
        min: Option<String>,
        /// 最大值
        max: Option<String>,
    },
    /// 必填字段缺失
    Required {
        /// 字段名
        field: String,
    },
    /// 值已存在
    AlreadyExists {
        /// 字段名
        field: String,
        /// 值
        value: String,
    },
    /// 值不允许
    NotAllowed {
        /// 字段名
        field: String,
        /// 允许的值
        allowed: Vec<String>,
    },
    /// 自定义验证错误
    CustomValidation {
        /// 字段名
        field: String,
        /// 错误消息
        message: String,
    },
}

impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationErrorKind::InvalidFormat { field, expected } => {
                write!(f, "invalid_format: field '{}' expected {}", field, expected)
            }
            ValidationErrorKind::OutOfRange { field, min, max } => {
                write!(f, "out_of_range: field '{}'", field)?;
                if let Some(min) = min {
                    write!(f, " min={}", min)?;
                }
                if let Some(max) = max {
                    write!(f, " max={}", max)?;
                }
                Ok(())
            }
            ValidationErrorKind::Required { field } => write!(f, "required: field '{}' is required", field),
            ValidationErrorKind::AlreadyExists { field, value } => {
                write!(f, "already_exists: field '{}' with value '{}' already exists", field, value)
            }
            ValidationErrorKind::NotAllowed { field, allowed } => {
                write!(f, "not_allowed: field '{}' must be one of {:?}", field, allowed)
            }
            ValidationErrorKind::CustomValidation { field, message } => {
                write!(f, "validation_error: field '{}' - {}", field, message)
            }
        }
    }
}

/// 网络错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkErrorKind {
    /// 连接失败
    ConnectionFailed,
    /// DNS 解析失败
    DnsFailed,
    /// DNS 解析失败
    DnsResolutionFailed,
    /// 请求超时
    Timeout,
    /// SSL/TLS 错误
    TlsError,
    /// 协议错误
    ProtocolError,
}

impl fmt::Display for NetworkErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkErrorKind::ConnectionFailed => write!(f, "connection_failed"),
            NetworkErrorKind::DnsFailed => write!(f, "dns_failed"),
            NetworkErrorKind::DnsResolutionFailed => write!(f, "dns_resolution_failed"),
            NetworkErrorKind::Timeout => write!(f, "timeout"),
            NetworkErrorKind::TlsError => write!(f, "tls_error"),
            NetworkErrorKind::ProtocolError => write!(f, "protocol_error"),
        }
    }
}

/// 存储错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageErrorKind {
    /// 读取失败
    ReadFailed,
    /// 写入失败
    WriteFailed,
    /// 删除失败
    DeleteFailed,
    /// 容量不足
    InsufficientCapacity,
    /// 数据损坏
    DataCorruption,
    /// 文件未找到
    FileNotFound,
}

impl fmt::Display for StorageErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageErrorKind::ReadFailed => write!(f, "read_failed"),
            StorageErrorKind::WriteFailed => write!(f, "write_failed"),
            StorageErrorKind::DeleteFailed => write!(f, "delete_failed"),
            StorageErrorKind::InsufficientCapacity => write!(f, "insufficient_capacity"),
            StorageErrorKind::DataCorruption => write!(f, "data_corruption"),
            StorageErrorKind::FileNotFound => write!(f, "file_not_found"),
        }
    }
}

/// 数据库错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseErrorKind {
    /// 连接失败
    ConnectionFailed {
        /// 错误消息
        message: String,
    },
    /// 查询失败
    QueryFailed {
        /// 查询语句
        query: Option<String>,
        /// 错误消息
        message: String,
    },
    /// 执行失败
    ExecuteFailed {
        /// 执行语句
        query: Option<String>,
        /// 错误消息
        message: String,
    },
    /// 事务失败
    TransactionFailed {
        /// 错误消息
        message: String,
    },
    /// 迁移失败
    MigrationFailed {
        /// 错误消息
        message: String,
    },
}

impl fmt::Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseErrorKind::ConnectionFailed { message } => {
                write!(f, "connection_failed: {}", message)
            }
            DatabaseErrorKind::QueryFailed { query, message } => {
                if let Some(q) = query {
                    write!(f, "query_failed ({}): {}", q, message)
                }
                else {
                    write!(f, "query_failed: {}", message)
                }
            }
            DatabaseErrorKind::ExecuteFailed { query, message } => {
                if let Some(q) = query {
                    write!(f, "execute_failed ({}): {}", q, message)
                }
                else {
                    write!(f, "execute_failed: {}", message)
                }
            }
            DatabaseErrorKind::TransactionFailed { message } => {
                write!(f, "transaction_failed: {}", message)
            }
            DatabaseErrorKind::MigrationFailed { message } => {
                write!(f, "migration_failed: {}", message)
            }
        }
    }
}

/// WAE 错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaeError {
    /// 错误类型
    pub kind: ErrorKind,
    /// 错误消息
    pub message: Option<String>,
    /// 附加参数
    pub params: std::collections::HashMap<String, String>,
}

impl WaeError {
    /// 创建新的错误
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind, message: None, params: std::collections::HashMap::new() }
    }

    /// 创建验证错误
    pub fn validation(kind: ValidationErrorKind) -> Self {
        Self::new(ErrorKind::Validation).with_param("validation_kind", kind.to_string())
    }

    /// 创建验证错误（简单消息）
    pub fn validation_message(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Validation).with_message(message)
    }

    /// 创建网络错误
    pub fn network(kind: NetworkErrorKind) -> Self {
        Self::new(ErrorKind::Network).with_param("network_kind", kind.to_string())
    }

    /// 创建存储错误
    pub fn storage(kind: StorageErrorKind) -> Self {
        Self::new(ErrorKind::Storage).with_param("storage_kind", kind.to_string())
    }

    /// 创建数据库错误
    pub fn database(kind: DatabaseErrorKind) -> Self {
        Self::new(ErrorKind::Internal).with_param("database_kind", kind.to_string())
    }

    /// 创建内部错误
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Internal).with_message(message)
    }

    /// 创建未找到错误
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound).with_param("resource", resource)
    }

    /// 创建权限错误
    pub fn permission(action: impl Into<String>) -> Self {
        Self::new(ErrorKind::Permission).with_param("action", action)
    }

    /// 创建超时错误
    pub fn timeout(operation: impl Into<String>) -> Self {
        Self::new(ErrorKind::Timeout).with_param("operation", operation)
    }

    /// 创建配置错误
    pub fn config(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Config).with_message(message)
    }

    /// 创建无效参数错误
    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Validation).with_message(message)
    }

    /// 设置错误消息
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// 添加参数
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// 获取参数
    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}

impl fmt::Display for WaeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{:?}: {}", self.kind, msg),
            None => write!(f, "{:?}", self.kind),
        }
    }
}

impl std::error::Error for WaeError {}

impl From<ValidationErrorKind> for WaeError {
    fn from(kind: ValidationErrorKind) -> Self {
        let message = match &kind {
            ValidationErrorKind::InvalidFormat { field, expected } => {
                format!("Invalid format for field '{}': expected {}", field, expected)
            }
            ValidationErrorKind::OutOfRange { field, min, max } => match (min, max) {
                (Some(min), Some(max)) => format!("Field '{}' out of range: expected between {} and {}", field, min, max),
                (Some(min), None) => format!("Field '{}' out of range: expected >= {}", field, min),
                (None, Some(max)) => format!("Field '{}' out of range: expected <= {}", field, max),
                (None, None) => format!("Field '{}' out of range", field),
            },
            ValidationErrorKind::Required { field } => format!("Field '{}' is required", field),
            ValidationErrorKind::AlreadyExists { field, value } => {
                format!("Field '{}' with value '{}' already exists", field, value)
            }
            ValidationErrorKind::NotAllowed { field, allowed } => {
                format!("Field '{}' value not allowed. Allowed values: {:?}", field, allowed)
            }
            ValidationErrorKind::CustomValidation { field, message } => {
                format!("Validation error for field '{}': {}", field, message)
            }
        };
        Self::new(ErrorKind::Validation).with_message(message)
    }
}

impl From<NetworkErrorKind> for WaeError {
    fn from(kind: NetworkErrorKind) -> Self {
        Self::network(kind)
    }
}

impl From<StorageErrorKind> for WaeError {
    fn from(kind: StorageErrorKind) -> Self {
        Self::storage(kind)
    }
}

impl From<DatabaseErrorKind> for WaeError {
    fn from(kind: DatabaseErrorKind) -> Self {
        Self::database(kind)
    }
}

/// WAE 结果类型
pub type WaeResult<T> = Result<T, WaeError>;

/// 云服务错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudError {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// HTTP 状态码
    pub status: Option<u16>,
    /// 附加详情
    pub details: Option<serde_json::Value>,
}

impl CloudError {
    /// 创建新的云服务错误
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into(), status: None, details: None }
    }

    /// 创建内部错误
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }

    /// 创建无效参数错误
    pub fn invalid_param(param: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::new("INVALID_PARAMETER", format!("Invalid parameter '{}': {}", param.into(), reason.into()))
    }

    /// 创建未找到错误
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::new("NOT_FOUND", format!("{} not found", resource.into()))
    }

    /// 创建权限错误
    pub fn permission_denied(action: impl Into<String>) -> Self {
        Self::new("PERMISSION_DENIED", format!("Permission denied for action: {}", action.into()))
    }

    /// 创建超时错误
    pub fn timeout(operation: impl Into<String>) -> Self {
        Self::new("TIMEOUT", format!("Operation timed out: {}", operation.into()))
    }

    /// 创建服务不可用错误
    pub fn service_unavailable(service: impl Into<String>) -> Self {
        Self::new("SERVICE_UNAVAILABLE", format!("Service unavailable: {}", service.into()))
    }

    /// 设置 HTTP 状态码
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置附加详情
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl fmt::Display for CloudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            Some(status) => write!(f, "[{}] {} (HTTP {})", self.code, self.message, status),
            None => write!(f, "[{}] {}", self.code, self.message),
        }
    }
}

impl std::error::Error for CloudError {}

impl From<WaeError> for CloudError {
    fn from(err: WaeError) -> Self {
        CloudError::new(format!("{:?}", err.kind), err.message.unwrap_or_default())
    }
}

/// 云服务结果类型
pub type CloudResult<T> = Result<T, CloudError>;
