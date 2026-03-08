//! Turso 数据库实现

use crate::connection::{
    config::{DatabaseConfig, DatabaseResult},
    row::DatabaseRows,
    statement::DatabaseStatement,
    trait_impl::DatabaseConnection,
};
use async_trait::async_trait;
use std::path::PathBuf;
use turso::{Builder, Connection, Database, Value as TursoValue};
use wae_types::{DatabaseErrorKind, WaeError};

/// Turso 连接包装
pub struct TursoConnection {
    conn: Connection,
}

impl TursoConnection {
    pub(crate) fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl DatabaseConnection for TursoConnection {
    async fn query(&self, sql: &str) -> DatabaseResult<DatabaseRows> {
        let mut stmt = self.conn.prepare(sql).await.map_err(|e| {
            WaeError::database(DatabaseErrorKind::QueryFailed {
                query: Some(sql.to_string()),
                reason: format!("Prepare failed: {}", e),
            })
        })?;
        let rows = stmt.query(()).await.map_err(|e| {
            WaeError::database(DatabaseErrorKind::QueryFailed {
                query: Some(sql.to_string()),
                reason: format!("Query failed: {}", e),
            })
        })?;
        Ok(DatabaseRows::new_turso(rows))
    }

    async fn query_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<DatabaseRows> {
        let turso_params = crate::types::from_wae_values(params);
        self.query_with_turso(sql, turso_params).await
    }

    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        self.conn.execute(sql, ()).await.map_err(|e| {
            WaeError::database(DatabaseErrorKind::ExecuteFailed {
                statement: Some(sql.to_string()),
                reason: format!("Execute failed: {}", e),
            })
        })
    }

    async fn execute_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<u64> {
        let turso_params = crate::types::from_wae_values(params);
        self.execute_with_turso(sql, turso_params).await
    }

    async fn prepare(&self, sql: &str) -> DatabaseResult<DatabaseStatement> {
        self.conn
            .prepare(sql)
            .await
            .map_err(|e| {
                WaeError::database(DatabaseErrorKind::QueryFailed {
                    query: Some(sql.to_string()),
                    reason: format!("Prepare failed: {}", e),
                })
            })
            .map(DatabaseStatement::new_turso)
    }

    async fn query_with_turso(&self, sql: &str, params: Vec<TursoValue>) -> DatabaseResult<DatabaseRows> {
        let mut stmt = self.conn.prepare(sql).await.map_err(|e| {
            WaeError::database(DatabaseErrorKind::QueryFailed {
                query: Some(sql.to_string()),
                reason: format!("Prepare failed: {}", e),
            })
        })?;
        let rows = stmt.query(params).await.map_err(|e| {
            WaeError::database(DatabaseErrorKind::QueryFailed {
                query: Some(sql.to_string()),
                reason: format!("Query failed: {}", e),
            })
        })?;
        Ok(DatabaseRows::new_turso(rows))
    }

    async fn execute_with_turso(&self, sql: &str, params: Vec<TursoValue>) -> DatabaseResult<u64> {
        self.conn.execute(sql, params).await.map_err(|e| {
            WaeError::database(DatabaseErrorKind::ExecuteFailed {
                statement: Some(sql.to_string()),
                reason: format!("Execute failed: {}", e),
            })
        })
    }
}

/// Turso 数据库服务
pub struct DatabaseService {
    db: Database,
}

impl DatabaseService {
    /// 从配置创建数据库服务
    pub async fn new(config: &DatabaseConfig) -> DatabaseResult<Self> {
        match config {
            DatabaseConfig::Turso { path } => {
                let db = match path {
                    Some(path) => {
                        let path_str = path.to_string_lossy().into_owned();
                        Builder::new_local(&path_str).build().await.map_err(|e| {
                            WaeError::database(DatabaseErrorKind::DatabaseConnectionFailed {
                                reason: format!("Failed to create database: {}", e),
                            })
                        })?
                    }
                    None => Builder::new_local(":memory:").build().await.map_err(|e| {
                        WaeError::database(DatabaseErrorKind::DatabaseConnectionFailed {
                            reason: format!("Failed to create database: {}", e),
                        })
                    })?,
                };
                Ok(Self { db })
            }
            #[cfg(feature = "postgres")]
            DatabaseConfig::Postgres { .. } => Err(WaeError::database(DatabaseErrorKind::DatabaseConnectionFailed {
                reason: "Use PostgresDatabaseService for Postgres".to_string(),
            })),
            #[cfg(feature = "mysql")]
            DatabaseConfig::MySql { .. } => Err(WaeError::database(DatabaseErrorKind::DatabaseConnectionFailed {
                reason: "Use MySqlDatabaseService for MySQL".to_string(),
            })),
        }
    }

    /// 创建内存数据库
    pub async fn in_memory() -> DatabaseResult<Self> {
        Self::new(&DatabaseConfig::turso_in_memory()).await
    }

    /// 创建文件数据库
    pub async fn file<P: Into<PathBuf>>(path: P) -> DatabaseResult<Self> {
        Self::new(&DatabaseConfig::turso_file(path)).await
    }

    /// 获取连接
    pub fn connect(&self) -> DatabaseResult<TursoConnection> {
        let conn = self.db.connect().map_err(|e| {
            WaeError::database(DatabaseErrorKind::DatabaseConnectionFailed { reason: format!("Failed to connect: {}", e) })
        })?;
        Ok(TursoConnection::new(conn))
    }
}
