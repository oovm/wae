//! 实体定义模块
//!
//! 提供实体与数据库表的映射关系定义

use crate::{
    DatabaseResult,
    connection::{DatabaseRow, FromDatabaseValue},
};
use wae_types::Value;

/// 数据库实体 trait
///
/// 定义实体与数据库表的映射关系
///
/// # Example
///
/// ```ignore
/// struct User {
///     id: i64,
///     name: String,
///     email: String,
/// }
///
/// impl Entity for User {
///     type Id = i64;
///
///     fn table_name() -> &'static str {
///         "users"
///     }
///
///     fn id(&self) -> Self::Id {
///         self.id
///     }
/// }
/// ```
pub trait Entity: Sized + Send + Sync + 'static {
    /// 主键类型
    type Id: FromDatabaseValue + Into<Value> + Send + Sync + Clone;

    /// 表名
    fn table_name() -> &'static str;

    /// 获取主键值
    fn id(&self) -> Self::Id;

    /// 主键列名，默认为 "id"
    fn id_column() -> &'static str {
        "id"
    }
}

/// 从数据库行解析实体
pub trait FromRow: Sized {
    /// 从数据库行解析
    fn from_row(row: &DatabaseRow) -> DatabaseResult<Self>;
}

/// 将实体转换为数据库行
pub trait ToRow {
    /// 转换为列名和值的列表
    fn to_row(&self) -> Vec<(&'static str, Value)>;
}
