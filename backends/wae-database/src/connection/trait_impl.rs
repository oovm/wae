//! 数据库连接抽象 trait

use crate::connection::{config::DatabaseResult, row::DatabaseRows, statement::DatabaseStatement};
use async_trait::async_trait;

/// 数据库连接抽象
#[async_trait]
pub trait DatabaseConnection: Send + Sync {
    /// 执行查询，返回结果集
    async fn query(&self, sql: &str) -> DatabaseResult<DatabaseRows>;

    /// 执行带参数的查询 (使用 wae_types::Value)
    async fn query_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<DatabaseRows>;

    /// 执行语句，返回影响的行数
    async fn execute(&self, sql: &str) -> DatabaseResult<u64>;

    /// 执行带参数的语句 (使用 wae_types::Value)
    async fn execute_with(&self, sql: &str, params: Vec<wae_types::Value>) -> DatabaseResult<u64>;

    /// 准备语句
    async fn prepare(&self, sql: &str) -> DatabaseResult<DatabaseStatement>;

    #[cfg(feature = "turso")]
    /// 执行带参数的查询 (内部使用 turso::Value)
    async fn query_with_turso(&self, _sql: &str, _params: Vec<turso::Value>) -> DatabaseResult<DatabaseRows> {
        unimplemented!("query_with_turso is only implemented for Turso connections")
    }

    #[cfg(feature = "turso")]
    /// 执行带参数的语句 (内部使用 turso::Value)
    async fn execute_with_turso(&self, _sql: &str, _params: Vec<turso::Value>) -> DatabaseResult<u64> {
        unimplemented!("execute_with_turso is only implemented for Turso connections")
    }
}
