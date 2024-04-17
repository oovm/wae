//! 预处理语句模块

use crate::connection::{config::DatabaseResult, row::DatabaseRows};
use wae_types::WaeError;

/// 预处理语句
pub enum DatabaseStatement {
    /// Turso 预处理语句
    #[cfg(feature = "turso")]
    Turso(turso::Statement),
    /// PostgreSQL 预处理语句
    #[cfg(feature = "postgres")]
    Postgres(String),
    /// MySQL 预处理语句
    #[cfg(feature = "mysql")]
    MySql(String),
}

impl DatabaseStatement {
    #[cfg(feature = "turso")]
    pub(crate) fn new_turso(stmt: turso::Statement) -> Self {
        Self::Turso(stmt)
    }

    #[cfg(feature = "postgres")]
    pub(crate) fn new_postgres(sql: String) -> Self {
        Self::Postgres(sql)
    }

    #[cfg(feature = "mysql")]
    pub(crate) fn new_mysql(sql: String) -> Self {
        Self::MySql(sql)
    }

    /// 执行查询 (使用 wae_types::Value)
    pub async fn query(&mut self, _params: Vec<wae_types::Value>) -> DatabaseResult<DatabaseRows> {
        match self {
            #[cfg(feature = "turso")]
            Self::Turso(stmt) => {
                let turso_params = crate::types::from_wae_values(_params);
                stmt.query(turso_params)
                    .await
                    .map_err(|e| WaeError::internal(format!("Query failed: {}", e)))
                    .map(DatabaseRows::new_turso)
            }
            #[cfg(feature = "postgres")]
            Self::Postgres(_) => Err(WaeError::internal("Prepared statements not supported for Postgres yet")),
            #[cfg(feature = "mysql")]
            Self::MySql(_) => Err(WaeError::internal("Prepared statements not supported for MySQL yet")),
        }
    }
}
