//! Schema 反射模块
//!
//! 从数据库读取现有表结构信息。

use std::collections::HashMap;
use wae_database::{ColumnDef, ColumnType, DatabaseConnection, DatabaseResult, IndexDef, TableSchema};
use wae_types::WaeResult;

/// Schema 反射器
pub struct SchemaReflector<'a> {
    conn: &'a dyn DatabaseConnection,
}

impl<'a> SchemaReflector<'a> {
    /// 创建新的反射器
    pub fn new(conn: &'a dyn DatabaseConnection) -> Self {
        Self { conn }
    }

    /// 获取所有表名
    pub async fn get_table_names(&self) -> DatabaseResult<Vec<String>> {
        let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name != '_migrations'";
        let mut rows = self.conn.query(sql).await?;

        let mut tables = Vec::new();
        while let Some(row) = rows.next().await? {
            tables.push(row.get::<String>(0)?);
        }

        Ok(tables)
    }

    /// 获取表的完整结构
    pub async fn get_table_schema(&self, table_name: &str) -> DatabaseResult<TableSchema> {
        let columns = self.get_columns(table_name).await?;
        let indexes = self.get_indexes(table_name).await?;

        Ok(TableSchema { name: table_name.to_string(), columns, indexes })
    }

    /// 获取表的所有列
    pub async fn get_columns(&self, table_name: &str) -> DatabaseResult<Vec<ColumnDef>> {
        let sql = format!("PRAGMA table_info({})", table_name);
        let mut rows = self.conn.query(&sql).await?;

        let mut columns = Vec::new();
        while let Some(row) = rows.next().await? {
            let name = row.get::<String>(1)?;
            let type_str = row.get::<String>(2)?;
            let not_null = row.get::<i64>(3)? != 0;
            let default_value = row.get::<Option<String>>(4)?;
            let is_pk = row.get::<i64>(5)? != 0;

            let col_type = ColumnType::from_sql(&type_str);

            columns.push(ColumnDef {
                name,
                col_type,
                nullable: !not_null,
                primary_key: is_pk,
                auto_increment: false,
                default_value,
                unique: false,
            });
        }

        Ok(columns)
    }

    /// 获取表的所有索引
    pub async fn get_indexes(&self, table_name: &str) -> DatabaseResult<Vec<IndexDef>> {
        let sql = format!("PRAGMA index_list({})", table_name);
        let mut rows = self.conn.query(&sql).await?;

        let mut indexes = Vec::new();
        while let Some(row) = rows.next().await? {
            let index_name = row.get::<String>(1)?;
            let unique = row.get::<i64>(2)? != 0;

            let columns = self.get_index_columns(&index_name).await?;

            indexes.push(IndexDef { name: index_name, table_name: table_name.to_string(), columns, unique });
        }

        Ok(indexes)
    }

    /// 获取索引的列
    async fn get_index_columns(&self, index_name: &str) -> DatabaseResult<Vec<String>> {
        let sql = format!("PRAGMA index_info({})", index_name);
        let mut rows = self.conn.query(&sql).await?;

        let mut columns = Vec::new();
        while let Some(row) = rows.next().await? {
            columns.push(row.get::<String>(2)?);
        }

        Ok(columns)
    }

    /// 获取所有表的 Schema
    pub async fn get_all_schemas(&self) -> WaeResult<HashMap<String, TableSchema>> {
        let table_names = self.get_table_names().await?;
        let mut schemas = HashMap::new();

        for name in table_names {
            let schema = self.get_table_schema(&name).await?;
            schemas.insert(name, schema);
        }

        Ok(schemas)
    }

    /// 检查表是否存在
    pub async fn table_exists(&self, table_name: &str) -> DatabaseResult<bool> {
        let sql = format!("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'", table_name);
        let mut rows = self.conn.query(&sql).await?;

        if let Some(row) = rows.next().await? {
            let count = row.get::<i64>(0)?;
            Ok(count > 0)
        }
        else {
            Ok(false)
        }
    }
}
