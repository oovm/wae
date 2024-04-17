//! 数据库配置模块

use serde::{Deserialize, Serialize};
#[cfg(feature = "turso")]
use std::path::PathBuf;
use wae_types::{WaeError, WaeResult};

/// 数据库操作结果类型
pub type DatabaseResult<T> = WaeResult<T>;

/// 数据库错误类型
pub type DatabaseError = WaeError;

/// 数据库配置枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseConfig {
    /// Turso 配置
    #[cfg(feature = "turso")]
    Turso {
        /// 数据库文件路径，为 None 时使用内存数据库
        path: Option<PathBuf>,
    },
    /// PostgreSQL 配置
    #[cfg(feature = "postgres")]
    Postgres {
        /// 连接字符串
        connection_string: String,
        /// 最大连接数
        max_connections: Option<usize>,
    },
    /// MySQL 配置
    #[cfg(feature = "mysql")]
    MySql {
        /// 连接字符串
        connection_string: String,
        /// 最大连接数
        max_connections: Option<usize>,
    },
}

impl DatabaseConfig {
    /// 创建 Turso 内存数据库配置
    #[cfg(feature = "turso")]
    pub fn turso_in_memory() -> Self {
        Self::Turso { path: None }
    }

    /// 创建 Turso 文件数据库配置
    #[cfg(feature = "turso")]
    pub fn turso_file<P: Into<PathBuf>>(path: P) -> Self {
        Self::Turso { path: Some(path.into()) }
    }

    /// 创建 PostgreSQL 配置
    #[cfg(feature = "postgres")]
    pub fn postgres<S: Into<String>>(connection_string: S) -> Self {
        Self::Postgres { connection_string: connection_string.into(), max_connections: None }
    }

    /// 创建 PostgreSQL 配置，带最大连接数
    #[cfg(feature = "postgres")]
    pub fn postgres_with_max_connections<S: Into<String>>(connection_string: S, max_connections: usize) -> Self {
        Self::Postgres { connection_string: connection_string.into(), max_connections: Some(max_connections) }
    }

    /// 创建 MySQL 配置
    #[cfg(feature = "mysql")]
    pub fn mysql<S: Into<String>>(connection_string: S) -> Self {
        Self::MySql { connection_string: connection_string.into(), max_connections: None }
    }

    /// 创建 MySQL 配置，带最大连接数
    #[cfg(feature = "mysql")]
    pub fn mysql_with_max_connections<S: Into<String>>(connection_string: S, max_connections: usize) -> Self {
        Self::MySql { connection_string: connection_string.into(), max_connections: Some(max_connections) }
    }
}
