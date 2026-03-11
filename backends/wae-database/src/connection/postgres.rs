//! PostgreSQL 数据库实现

use crate::connection::{
    config::{DatabaseConfig, DatabaseResult},
    row::DatabaseRows,
    statement::DatabaseStatement,
    trait_impl::{DatabaseBackend, DatabaseConnection},
};
use async_trait::async_trait;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{
    Config, NoTls,
    types::{ToSql, private::BytesMut},
};
use wae_types::{WaeError, WaeErrorKind};

/// PostgreSQL 参数值
///
/// 使用枚举来避免 trait object 的 Send 问题
#[derive(Debug, Clone)]
pub(crate) enum PostgresParam {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
}

impl ToSql for PostgresParam {
    fn to_sql(
        &self,
        ty: &tokio_postgres::types::Type,
        out: &mut BytesMut,
    ) -> std::result::Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match self {
            PostgresParam::Null => Option::<i32>::None.to_sql(ty, out),
            PostgresParam::Bool(b) => b.to_sql(ty, out),
            PostgresParam::Int(i) => i.to_sql(ty, out),
            PostgresParam::Float(f) => f.to_sql(ty, out),
            PostgresParam::String(s) => s.to_sql(ty, out),
            PostgresParam::Bytes(b) => b.to_sql(ty, out),
        }
    }

    fn accepts(_ty: &tokio_postgres::types::Type) -> bool {
        true
    }

    tokio_postgres::types::to_sql_checked!();
}

pub(crate) fn value_to_postgres_param(value: wae_types::Value) -> PostgresParam {
    match value {
        wae_types::Value::Null => PostgresParam::Null,
        wae_types::Value::Bool(b) => PostgresParam::Bool(b),
        wae_types::Value::Integer(i) => PostgresParam::Int(i),
        wae_types::Value::Float(f) => PostgresParam::Float(f),
        wae_types::Value::String(s) => PostgresParam::String(s),
        wae_types::Value::Bytes(b) => PostgresParam::Bytes(b),
        wae_types::Value::Array(l) => PostgresParam::String(serde_json::to_string(&l).unwrap_or_default()),
        wae_types::Value::Object(m) => PostgresParam::String(serde_json::to_string(&m).unwrap_or_default()),
    }
}

/// PostgreSQL 连接包装
pub struct PostgresConnection {
    client: deadpool_postgres::Client,
}

impl PostgresConnection {
    pub(crate) fn new(client: deadpool_postgres::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl DatabaseConnection for PostgresConnection {
    fn backend(&self) -> DatabaseBackend {
        DatabaseBackend::Postgres
    }

    async fn query(&self, sql: &str) -> DatabaseResult<DatabaseRows> {
        let rows = self.client.query(sql, &[]).await.map_err(|e| {
            WaeError::database(WaeErrorKind::QueryFailed { query: Some(sql.to_string()), reason: e.to_string() })
        })?;
        Ok(DatabaseRows::new_postgres(rows))
    }

    async fn query_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<DatabaseRows> {
        let postgres_params: Vec<PostgresParam> = params.into_iter().map(value_to_postgres_param).collect();
        let postgres_refs: Vec<&(dyn ToSql + Sync)> = postgres_params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();
        let rows = self.client.query(sql, postgres_refs.as_slice()).await.map_err(|e| {
            WaeError::database(WaeErrorKind::QueryFailed { query: Some(sql.to_string()), reason: e.to_string() })
        })?;
        Ok(DatabaseRows::new_postgres(rows))
    }

    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        let affected = self.client.execute(sql, &[]).await.map_err(|e| {
            WaeError::database(WaeErrorKind::ExecuteFailed { statement: Some(sql.to_string()), reason: e.to_string() })
        })?;
        Ok(affected as u64)
    }

    async fn execute_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<u64> {
        let postgres_params: Vec<PostgresParam> = params.into_iter().map(value_to_postgres_param).collect();
        let postgres_refs: Vec<&(dyn ToSql + Sync)> = postgres_params.iter().map(|p| p as &(dyn ToSql + Sync)).collect();
        let affected = self.client.execute(sql, postgres_refs.as_slice()).await.map_err(|e| {
            WaeError::database(WaeErrorKind::ExecuteFailed { statement: Some(sql.to_string()), reason: e.to_string() })
        })?;
        Ok(affected as u64)
    }

    async fn prepare(&self, sql: &str) -> DatabaseResult<DatabaseStatement> {
        Ok(DatabaseStatement::new_postgres(sql.to_string()))
    }

    async fn begin_transaction(&self) -> DatabaseResult<()> {
        self.client.execute("BEGIN TRANSACTION", &[]).await.map_err(|e| {
            WaeError::database(WaeErrorKind::TransactionFailed { reason: format!("Failed to begin transaction: {}", e) })
        })?;
        Ok(())
    }

    async fn commit(&self) -> DatabaseResult<()> {
        self.client.execute("COMMIT", &[]).await.map_err(|e| {
            WaeError::database(WaeErrorKind::TransactionFailed { reason: format!("Failed to commit transaction: {}", e) })
        })?;
        Ok(())
    }

    async fn rollback(&self) -> DatabaseResult<()> {
        self.client.execute("ROLLBACK", &[]).await.map_err(|e| {
            WaeError::database(WaeErrorKind::TransactionFailed { reason: format!("Failed to rollback transaction: {}", e) })
        })?;
        Ok(())
    }
}

/// PostgreSQL 数据库服务
pub struct PostgresDatabaseService {
    pool: Pool,
}

impl PostgresDatabaseService {
    /// 创建 PostgreSQL 数据库服务实例
    pub async fn new(config: &DatabaseConfig) -> DatabaseResult<Self> {
        match config {
            #[cfg(feature = "turso")]
            DatabaseConfig::Turso { .. } => Err(WaeError::database(WaeErrorKind::DatabaseConnectionFailed {
                reason: "Use DatabaseService for Turso".to_string(),
            })),
            DatabaseConfig::Postgres { connection_string, max_connections } => {
                let config: Config = connection_string.parse().map_err(|e| {
                    WaeError::database(WaeErrorKind::DatabaseConnectionFailed {
                        reason: format!("Invalid connection string: {}", e),
                    })
                })?;
                let manager_config = ManagerConfig { recycling_method: RecyclingMethod::Fast };
                let manager = Manager::from_config(config, NoTls, manager_config);
                let max_size = max_connections.unwrap_or(10);
                let pool = Pool::builder(manager).max_size(max_size).build().map_err(|e| {
                    WaeError::database(WaeErrorKind::DatabaseConnectionFailed {
                        reason: format!("Failed to create pool: {}", e),
                    })
                })?;
                Ok(Self { pool })
            }
            #[cfg(feature = "mysql")]
            DatabaseConfig::MySql { .. } => Err(WaeError::database(WaeErrorKind::DatabaseConnectionFailed {
                reason: "Use MySqlDatabaseService for MySQL".to_string(),
            })),
        }
    }

    /// 从连接池获取数据库连接
    pub async fn connect(&self) -> DatabaseResult<PostgresConnection> {
        let client = self.pool.get().await.map_err(|e| {
            WaeError::database(WaeErrorKind::DatabaseConnectionFailed { reason: format!("Failed to get connection: {}", e) })
        })?;
        Ok(PostgresConnection::new(client))
    }
}
