//! Schema 定义模块
//!
//! 提供数据库表结构的元信息定义，用于自动迁移。

use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;

/// 列类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    /// 整数类型
    Integer,
    /// 浮点类型
    Real,
    /// 文本类型
    Text,
    /// 二进制类型
    Blob,
}

impl ColumnType {
    /// 转换为 SQL 类型字符串
    pub fn to_sql(&self) -> &'static str {
        match self {
            ColumnType::Integer => "INTEGER",
            ColumnType::Real => "REAL",
            ColumnType::Text => "TEXT",
            ColumnType::Blob => "BLOB",
        }
    }

    /// 转换为 MySQL SQL 类型字符串
    #[cfg(feature = "mysql")]
    pub fn to_mysql_sql(&self) -> &'static str {
        match self {
            ColumnType::Integer => "BIGINT",
            ColumnType::Real => "DOUBLE",
            ColumnType::Text => "VARCHAR(255)",
            ColumnType::Blob => "BLOB",
        }
    }

    /// 从 SQL 类型字符串解析
    pub fn from_sql(sql: &str) -> Self {
        match sql.to_uppercase().as_str() {
            "INTEGER" | "INT" | "BIGINT" | "SMALLINT" | "TINYINT" => ColumnType::Integer,
            "REAL" | "FLOAT" | "DOUBLE" | "NUMERIC" | "DECIMAL" => ColumnType::Real,
            "TEXT" | "VARCHAR" | "CHAR" | "STRING" => ColumnType::Text,
            "BLOB" | "BINARY" => ColumnType::Blob,
            _ => ColumnType::Text,
        }
    }
}

/// 列定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    /// 列名
    pub name: String,
    /// 列类型
    pub col_type: ColumnType,
    /// 是否可空
    pub nullable: bool,
    /// 是否主键
    pub primary_key: bool,
    /// 是否自增
    pub auto_increment: bool,
    /// 默认值
    pub default_value: Option<String>,
    /// 是否唯一
    pub unique: bool,
}

impl ColumnDef {
    /// 创建新的列定义
    pub fn new(name: impl Into<String>, col_type: ColumnType) -> Self {
        Self {
            name: name.into(),
            col_type,
            nullable: true,
            primary_key: false,
            auto_increment: false,
            default_value: None,
            unique: false,
        }
    }

    /// 设置为不可空
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    /// 设置为主键
    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self.nullable = false;
        self
    }

    /// 设置为自增
    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }

    /// 设置默认值
    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// 设置为唯一
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// 生成建表 SQL 片段
    pub fn to_sql(&self) -> String {
        let mut sql = format!("{} {}", self.name, self.col_type.to_sql());

        if self.primary_key {
            sql.push_str(" PRIMARY KEY");
        }

        if self.auto_increment {
            sql.push_str(" AUTOINCREMENT");
        }

        if !self.nullable && !self.primary_key {
            sql.push_str(" NOT NULL");
        }

        if let Some(ref default) = self.default_value {
            sql.push_str(&format!(" DEFAULT {}", default));
        }

        if self.unique && !self.primary_key {
            sql.push_str(" UNIQUE");
        }

        sql
    }

    /// 生成 MySQL 建表 SQL 片段
    #[cfg(feature = "mysql")]
    pub fn to_mysql_sql(&self) -> String {
        let mut sql = format!("{} {}", self.name, self.col_type.to_mysql_sql());

        if self.primary_key {
            sql.push_str(" PRIMARY KEY");
        }

        if self.auto_increment {
            sql.push_str(" AUTO_INCREMENT");
        }

        if !self.nullable && !self.primary_key {
            sql.push_str(" NOT NULL");
        }

        if let Some(ref default) = self.default_value {
            sql.push_str(&format!(" DEFAULT {}", default));
        }

        if self.unique && !self.primary_key {
            sql.push_str(" UNIQUE");
        }

        sql
    }
}

/// 索引定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDef {
    /// 索引名称
    pub name: String,
    /// 表名
    pub table_name: String,
    /// 列名列表
    pub columns: Vec<String>,
    /// 是否唯一索引
    pub unique: bool,
}

impl IndexDef {
    /// 创建新的索引定义
    pub fn new(name: impl Into<String>, table_name: impl Into<String>, columns: Vec<String>) -> Self {
        Self { name: name.into(), table_name: table_name.into(), columns, unique: false }
    }

    /// 设置为唯一索引
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// 生成创建索引 SQL
    pub fn to_create_sql(&self) -> String {
        let unique_str = if self.unique { "UNIQUE " } else { "" };
        let columns = self.columns.join(", ");
        format!("CREATE {}INDEX {} ON {} ({})", unique_str, self.name, self.table_name, columns)
    }

    /// 生成删除索引 SQL
    pub fn to_drop_sql(&self) -> String {
        format!("DROP INDEX IF EXISTS {}", self.name)
    }
}

/// 表结构定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    /// 表名
    pub name: String,
    /// 列定义列表
    pub columns: Vec<ColumnDef>,
    /// 索引定义列表
    pub indexes: Vec<IndexDef>,
}

impl TableSchema {
    /// 创建新的表结构
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), columns: Vec::new(), indexes: Vec::new() }
    }

    /// 添加列
    pub fn column(mut self, col: ColumnDef) -> Self {
        self.columns.push(col);
        self
    }

    /// 添加索引
    pub fn index(mut self, idx: IndexDef) -> Self {
        self.indexes.push(idx);
        self
    }

    /// 生成建表 SQL
    pub fn to_create_sql(&self) -> String {
        let columns: Vec<String> = self.columns.iter().map(|c| c.to_sql()).collect();
        format!("CREATE TABLE {} ({})", self.name, columns.join(", "))
    }

    /// 生成 MySQL 建表 SQL
    #[cfg(feature = "mysql")]
    pub fn to_mysql_create_sql(&self) -> String {
        let columns: Vec<String> = self.columns.iter().map(|c| c.to_mysql_sql()).collect();
        format!("CREATE TABLE {} ({}) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4", self.name, columns.join(", "))
    }

    /// 生成删表 SQL
    pub fn to_drop_sql(&self) -> String {
        format!("DROP TABLE IF EXISTS {}", self.name)
    }

    /// 获取主键列
    pub fn primary_key(&self) -> Option<&ColumnDef> {
        self.columns.iter().find(|c| c.primary_key)
    }

    /// 根据名称获取列
    pub fn get_column(&self, name: &str) -> Option<&ColumnDef> {
        self.columns.iter().find(|c| c.name == name)
    }
}

/// 外键定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyDef {
    /// 外键名称
    pub name: String,
    /// 本表列名
    pub column: String,
    /// 引用表名
    pub ref_table: String,
    /// 引用列名
    pub ref_column: String,
    /// 更新行为
    pub on_update: ReferentialAction,
    /// 删除行为
    pub on_delete: ReferentialAction,
}

/// 外键引用行为
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferentialAction {
    /// 无操作
    NoAction,
    /// 限制
    Restrict,
    /// 级联
    Cascade,
    /// 设为空
    SetNull,
    /// 设为默认值
    SetDefault,
}

impl ReferentialAction {
    /// 转换为 SQL
    pub fn to_sql(&self) -> &'static str {
        match self {
            ReferentialAction::NoAction => "NO ACTION",
            ReferentialAction::Restrict => "RESTRICT",
            ReferentialAction::Cascade => "CASCADE",
            ReferentialAction::SetNull => "SET NULL",
            ReferentialAction::SetDefault => "SET DEFAULT",
        }
    }
}

/// 列类型快捷构造函数
pub mod col {
    use super::{ColumnDef, ColumnType};

    /// 创建整数列
    pub fn integer(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Integer)
    }

    /// 创建浮点列
    pub fn real(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Real)
    }

    /// 创建文本列
    pub fn text(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Text)
    }

    /// 创建二进制列
    pub fn blob(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Blob)
    }

    /// 创建主键列 (自增整数)
    pub fn id(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Integer).primary_key().auto_increment()
    }

    /// 创建布尔列
    pub fn boolean(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Integer).default("0")
    }

    /// 创建时间戳列
    pub fn timestamp(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Text)
    }

    /// 创建 JSON 列
    pub fn json(name: &str) -> ColumnDef {
        ColumnDef::new(name, ColumnType::Text)
    }
}
