//! MySQL 数据库实现

use crate::connection::{
    config::{DatabaseConfig, DatabaseResult},
    row::DatabaseRows,
    statement::DatabaseStatement,
    trait_impl::DatabaseConnection,
};
use async_trait::async_trait;
use mysql_async::prelude::*;
use tokio::sync::Mutex;
use wae_types::WaeError;

/// MySQL 连接包装
pub struct MySqlConnection {
    conn: Mutex<Option<mysql_async::Conn>>,
}

impl MySqlConnection {
    pub(crate) fn new(conn: mysql_async::Conn) -> Self {
        Self { conn: Mutex::new(Some(conn)) }
    }
}

#[async_trait]
impl DatabaseConnection for MySqlConnection {
    async fn query(&self, sql: &str) -> DatabaseResult<DatabaseRows> {
        let mut guard = self.conn.lock().await;
        let conn = guard.as_mut().ok_or_else(|| WaeError::internal("Connection closed"))?;
        let result = conn.query_iter(sql).await.map_err(|e| WaeError::internal(format!("Query failed: {}", e)))?;
        let rows: Vec<mysql_async::Row> =
            result.collect_and_drop().await.map_err(|e| WaeError::internal(format!("Failed to collect rows: {}", e)))?;
        Ok(DatabaseRows::new_mysql(rows))
    }

    async fn query_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<DatabaseRows> {
        let mut guard = self.conn.lock().await;
        let conn = guard.as_mut().ok_or_else(|| WaeError::internal("Connection closed"))?;
        let mysql_params = crate::types::to_mysql_params(params);
        let result = conn.exec_iter(sql, mysql_params).await.map_err(|e| WaeError::internal(format!("Query failed: {}", e)))?;
        let rows: Vec<mysql_async::Row> =
            result.collect_and_drop().await.map_err(|e| WaeError::internal(format!("Failed to collect rows: {}", e)))?;
        Ok(DatabaseRows::new_mysql(rows))
    }

    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        let mut guard = self.conn.lock().await;
        let conn = guard.as_mut().ok_or_else(|| WaeError::internal("Connection closed"))?;
        let result = conn.query_iter(sql).await.map_err(|e| WaeError::internal(format!("Execute failed: {}", e)))?;
        let affected = result.affected_rows();
        result.drop_result().await.map_err(|e| WaeError::internal(format!("Failed to drop result: {}", e)))?;
        Ok(affected)
    }

    async fn execute_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<u64> {
        let mut guard = self.conn.lock().await;
        let conn = guard.as_mut().ok_or_else(|| WaeError::internal("Connection closed"))?;
        let mysql_params = crate::types::to_mysql_params(params);
        let result =
            conn.exec_iter(sql, mysql_params).await.map_err(|e| WaeError::internal(format!("Execute failed: {}", e)))?;
        let affected = result.affected_rows();
        result.drop_result().await.map_err(|e| WaeError::internal(format!("Failed to drop result: {}", e)))?;
        Ok(affected)
    }

    async fn prepare(&self, sql: &str) -> DatabaseResult<DatabaseStatement> {
        Ok(DatabaseStatement::new_mysql(sql.to_string()))
    }
}

/// MySQL 数据库服务
pub struct MySqlDatabaseService {
    pool: mysql_async::Pool,
}

impl MySqlDatabaseService {
    /// 从配置创建数据库服务
    pub async fn new(config: &DatabaseConfig) -> DatabaseResult<Self> {
        match config {
            #[cfg(feature = "turso")]
            DatabaseConfig::Turso { .. } => Err(WaeError::internal("Use DatabaseService for Turso")),
            #[cfg(feature = "postgres")]
            DatabaseConfig::Postgres { .. } => Err(WaeError::internal("Use PostgresDatabaseService for Postgres")),
            DatabaseConfig::MySql { connection_string, max_connections } => {
                let opts = mysql_async::Opts::from_url(connection_string)
                    .map_err(|e| WaeError::internal(format!("Invalid connection string: {}", e)))?;
                let constraints = mysql_async::PoolConstraints::new(1, max_connections.unwrap_or(10))
                    .ok_or_else(|| WaeError::internal("Invalid pool constraints: min must be <= max"))?;
                let pool_opts = mysql_async::PoolOpts::default().with_constraints(constraints);
                let opts = mysql_async::OptsBuilder::from_opts(opts).pool_opts(pool_opts);
                let pool = mysql_async::Pool::new(opts);
                Ok(Self { pool })
            }
        }
    }

    /// 获取连接
    pub async fn connect(&self) -> DatabaseResult<MySqlConnection> {
        let conn = self.pool.get_conn().await.map_err(|e| WaeError::internal(format!("Failed to get connection: {}", e)))?;
        Ok(MySqlConnection::new(conn))
    }
}
