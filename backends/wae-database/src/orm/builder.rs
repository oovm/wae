//! 查询构建器模块
//!
//! 提供 SELECT、INSERT、UPDATE、DELETE 查询构建器

use super::{
    condition::{Condition, Order},
    entity::{Entity, ToRow},
};
use std::marker::PhantomData;
use wae_types::Value;

#[cfg(feature = "turso")]
use crate::types::from_wae_value;
#[cfg(feature = "turso")]
use turso::Value as TursoValue;

#[cfg(feature = "mysql")]
use crate::types::from_wae_to_mysql;
#[cfg(feature = "mysql")]
use mysql_async::Value as MySqlValue;

/// SELECT 查询构建器
pub struct SelectBuilder<E: Entity> {
    columns: Vec<String>,
    conditions: Vec<Condition>,
    order_by: Vec<(String, Order)>,
    limit: Option<u64>,
    offset: Option<u64>,
    _marker: PhantomData<E>,
}

impl<E: Entity> SelectBuilder<E> {
    pub(crate) fn new() -> Self {
        Self {
            columns: Vec::new(),
            conditions: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _marker: PhantomData,
        }
    }

    /// 选择指定列
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// 添加 WHERE 条件
    pub fn where_(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// 添加多个 WHERE 条件 (AND 连接)
    pub fn where_all(mut self, conditions: Vec<Condition>) -> Self {
        self.conditions.extend(conditions);
        self
    }

    /// 添加排序
    pub fn order_by<C: Into<String>>(mut self, column: C, order: Order) -> Self {
        self.order_by.push((column.into(), order));
        self
    }

    /// 设置 LIMIT
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 设置 OFFSET
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    #[cfg(feature = "turso")]
    /// 构建 SQL 和参数 (内部使用)
    pub(crate) fn build_turso(&self) -> (String, Vec<TursoValue>) {
        let columns = if self.columns.is_empty() { "*".to_string() } else { self.columns.join(", ") };

        let mut sql = format!("SELECT {} FROM {}", columns, E::table_name());
        let mut params = Vec::new();

        if !self.conditions.is_empty() {
            let where_conditions = self.conditions.to_vec();
            if where_conditions.len() == 1 {
                let (cond_sql, cond_params) = where_conditions[0].build_turso();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
            else {
                let (cond_sql, cond_params) = Condition::and(where_conditions).build_turso();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
        }

        if !self.order_by.is_empty() {
            let order_parts: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, order)| {
                    format!(
                        "{} {}",
                        col,
                        match order {
                            Order::Asc => "ASC",
                            Order::Desc => "DESC",
                        }
                    )
                })
                .collect();
            sql.push_str(&format!(" ORDER BY {}", order_parts.join(", ")));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        (sql, params)
    }

    #[cfg(feature = "mysql")]
    #[allow(dead_code)]
    /// 构建 SQL 和参数 (内部使用 MySQL)
    pub(crate) fn build_mysql(&self) -> (String, Vec<MySqlValue>) {
        let columns = if self.columns.is_empty() { "*".to_string() } else { self.columns.join(", ") };

        let mut sql = format!("SELECT {} FROM {}", columns, E::table_name());
        let mut params = Vec::new();

        if !self.conditions.is_empty() {
            let where_conditions = self.conditions.to_vec();
            if where_conditions.len() == 1 {
                let (cond_sql, cond_params) = where_conditions[0].build_mysql();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
            else {
                let (cond_sql, cond_params) = Condition::and(where_conditions).build_mysql();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
        }

        if !self.order_by.is_empty() {
            let order_parts: Vec<String> = self
                .order_by
                .iter()
                .map(|(col, order)| {
                    format!(
                        "{} {}",
                        col,
                        match order {
                            Order::Asc => "ASC",
                            Order::Desc => "DESC",
                        }
                    )
                })
                .collect();
            sql.push_str(&format!(" ORDER BY {}", order_parts.join(", ")));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        (sql, params)
    }
}

/// INSERT 查询构建器
pub struct InsertBuilder<E: Entity> {
    data: Vec<(&'static str, Value)>,
    _marker: PhantomData<E>,
}

impl<E: Entity> InsertBuilder<E> {
    pub(crate) fn new() -> Self {
        Self { data: Vec::new(), _marker: PhantomData }
    }

    /// 从实体创建
    pub fn from_entity<T: ToRow>(entity: &T) -> Self {
        Self { data: entity.to_row(), _marker: PhantomData }
    }

    /// 添加列值
    pub fn value(mut self, column: &'static str, value: Value) -> Self {
        self.data.push((column, value));
        self
    }

    /// 批量添加列值
    pub fn values(mut self, data: Vec<(&'static str, Value)>) -> Self {
        self.data.extend(data);
        self
    }

    #[cfg(feature = "turso")]
    /// 构建 SQL 和参数 (内部使用)
    pub(crate) fn build_turso(&self) -> (String, Vec<TursoValue>) {
        let columns: Vec<&str> = self.data.iter().map(|(col, _)| *col).collect();
        let placeholders: Vec<&str> = self.data.iter().map(|_| "?").collect();
        let params: Vec<TursoValue> = self.data.iter().map(|(_, val)| from_wae_value(val.clone())).collect();

        let sql = format!("INSERT INTO {} ({}) VALUES ({})", E::table_name(), columns.join(", "), placeholders.join(", "));

        (sql, params)
    }

    #[cfg(feature = "mysql")]
    /// 构建 SQL 和参数 (内部使用 MySQL)
    pub(crate) fn build_mysql(&self) -> (String, Vec<MySqlValue>) {
        let columns: Vec<&str> = self.data.iter().map(|(col, _)| *col).collect();
        let placeholders: Vec<&str> = self.data.iter().map(|_| "?").collect();
        let params: Vec<MySqlValue> = self.data.iter().map(|(_, val)| from_wae_to_mysql(val.clone())).collect();

        let sql = format!("INSERT INTO {} ({}) VALUES ({})", E::table_name(), columns.join(", "), placeholders.join(", "));

        (sql, params)
    }
}

/// UPDATE 查询构建器
pub struct UpdateBuilder<E: Entity> {
    data: Vec<(&'static str, Value)>,
    conditions: Vec<Condition>,
    _marker: PhantomData<E>,
}

impl<E: Entity> UpdateBuilder<E> {
    pub(crate) fn new() -> Self {
        Self { data: Vec::new(), conditions: Vec::new(), _marker: PhantomData }
    }

    /// 设置列值
    pub fn set(mut self, column: &'static str, value: Value) -> Self {
        self.data.push((column, value));
        self
    }

    /// 批量设置列值
    pub fn set_all(mut self, data: Vec<(&'static str, Value)>) -> Self {
        self.data.extend(data);
        self
    }

    /// 从实体设置值 (排除主键)
    pub fn from_entity<T: ToRow + Entity>(entity: &T) -> Self {
        let id_col = T::id_column();
        let data: Vec<(&'static str, Value)> = entity.to_row().into_iter().filter(|(col, _)| *col != id_col).collect();
        Self { data, conditions: Vec::new(), _marker: PhantomData }
    }

    /// 添加 WHERE 条件
    pub fn where_(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// 按主键更新
    pub fn where_id(mut self, id: E::Id) -> Self {
        self.conditions.push(Condition::eq(E::id_column(), id));
        self
    }

    #[cfg(feature = "turso")]
    /// 构建 SQL 和参数 (内部使用)
    pub(crate) fn build_turso(&self) -> (String, Vec<TursoValue>) {
        let set_parts: Vec<String> = self.data.iter().map(|(col, _)| format!("{} = ?", col)).collect();
        let mut params: Vec<TursoValue> = self.data.iter().map(|(_, val)| from_wae_value(val.clone())).collect();

        let mut sql = format!("UPDATE {} SET {}", E::table_name(), set_parts.join(", "));

        if !self.conditions.is_empty() {
            let where_conditions = self.conditions.to_vec();
            if where_conditions.len() == 1 {
                let (cond_sql, cond_params) = where_conditions[0].build_turso();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
            else {
                let (cond_sql, cond_params) = Condition::and(where_conditions).build_turso();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
        }

        (sql, params)
    }

    #[cfg(feature = "mysql")]
    /// 构建 SQL 和参数 (内部使用 MySQL)
    pub(crate) fn build_mysql(&self) -> (String, Vec<MySqlValue>) {
        let set_parts: Vec<String> = self.data.iter().map(|(col, _)| format!("{} = ?", col)).collect();
        let mut params: Vec<MySqlValue> = self.data.iter().map(|(_, val)| from_wae_to_mysql(val.clone())).collect();

        let mut sql = format!("UPDATE {} SET {}", E::table_name(), set_parts.join(", "));

        if !self.conditions.is_empty() {
            let where_conditions = self.conditions.to_vec();
            if where_conditions.len() == 1 {
                let (cond_sql, cond_params) = where_conditions[0].build_mysql();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
            else {
                let (cond_sql, cond_params) = Condition::and(where_conditions).build_mysql();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
        }

        (sql, params)
    }
}

/// DELETE 查询构建器
pub struct DeleteBuilder<E: Entity> {
    conditions: Vec<Condition>,
    _marker: PhantomData<E>,
}

impl<E: Entity> DeleteBuilder<E> {
    pub(crate) fn new() -> Self {
        Self { conditions: Vec::new(), _marker: PhantomData }
    }

    /// 添加 WHERE 条件
    pub fn where_(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// 按主键删除
    pub fn where_id(mut self, id: E::Id) -> Self {
        self.conditions.push(Condition::eq(E::id_column(), id));
        self
    }

    #[cfg(feature = "turso")]
    /// 构建 SQL 和参数 (内部使用)
    pub(crate) fn build_turso(&self) -> (String, Vec<TursoValue>) {
        let mut sql = format!("DELETE FROM {}", E::table_name());
        let mut params = Vec::new();

        if !self.conditions.is_empty() {
            let where_conditions = self.conditions.to_vec();
            if where_conditions.len() == 1 {
                let (cond_sql, cond_params) = where_conditions[0].build_turso();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
            else {
                let (cond_sql, cond_params) = Condition::and(where_conditions).build_turso();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
        }

        (sql, params)
    }

    #[cfg(feature = "mysql")]
    /// 构建 SQL 和参数 (内部使用 MySQL)
    pub(crate) fn build_mysql(&self) -> (String, Vec<MySqlValue>) {
        let mut sql = format!("DELETE FROM {}", E::table_name());
        let mut params = Vec::new();

        if !self.conditions.is_empty() {
            let where_conditions = self.conditions.to_vec();
            if where_conditions.len() == 1 {
                let (cond_sql, cond_params) = where_conditions[0].build_mysql();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
            else {
                let (cond_sql, cond_params) = Condition::and(where_conditions).build_mysql();
                sql.push_str(&format!(" WHERE {}", cond_sql));
                params.extend(cond_params);
            }
        }

        (sql, params)
    }
}

/// 查询构建器入口
pub struct QueryBuilder;

impl QueryBuilder {
    /// 创建 SELECT 构建器
    pub fn select<E: Entity>() -> SelectBuilder<E> {
        SelectBuilder::new()
    }

    /// 创建 INSERT 构建器
    pub fn insert<E: Entity>() -> InsertBuilder<E> {
        InsertBuilder::new()
    }

    /// 创建 UPDATE 构建器
    pub fn update<E: Entity>() -> UpdateBuilder<E> {
        UpdateBuilder::new()
    }

    /// 创建 DELETE 构建器
    pub fn delete<E: Entity>() -> DeleteBuilder<E> {
        DeleteBuilder::new()
    }
}
