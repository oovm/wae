//! Schema 同步模块
//!
//! 提供从 schemas.yaml 到数据库的同步功能。

use wae_database::{DatabaseType, DatabaseLinkConfig, SchemaConfig, TableSchema, load_schema_config_from_yaml_file};
use wae_types::{WaeError, WaeResult};
use std::collections::HashMap;

/// 迁移操作类型
#[derive(Debug, Clone)]
pub enum MigrationOperation {
    /// 创建表
    CreateTable { schema: TableSchema },
    /// 删除表
    DropTable { table_name: String },
    /// 添加列
    AddColumn { table_name: String, column: wae_database::ColumnDef },
    /// 删除列
    DropColumn { table_name: String, column_name: String },
    /// 修改列
    AlterColumn { table_name: String, column: wae_database::ColumnDef },
    /// 创建索引
    CreateIndex { table_name: String, index: wae_database::IndexDef },
    /// 删除索引
    DropIndex { table_name: String, index_name: String },
}

/// 迁移操作描述辅助函数
pub fn operation_description(op: &MigrationOperation) -> String {
    match op {
        MigrationOperation::CreateTable { schema } => {
            format!("Create table '{}'", schema.name)
        }
        MigrationOperation::DropTable { table_name } => {
            format!("Drop table '{}'", table_name)
        }
        MigrationOperation::AddColumn { table_name, column } => {
            format!("Add column '{}' to table '{}'", column.name, table_name)
        }
        MigrationOperation::DropColumn { table_name, column_name } => {
            format!("Drop column '{}' from table '{}'", column_name, table_name)
        }
        MigrationOperation::AlterColumn { table_name, column } => {
            format!("Alter column '{}' in table '{}'", column.name, table_name)
        }
        MigrationOperation::CreateIndex { table_name, index } => {
            format!("Create index '{}' on table '{}'", index.name, table_name)
        }
        MigrationOperation::DropIndex { table_name, index_name } => {
            format!("Drop index '{}' from table '{}'", index_name, table_name)
        }
    }
}

/// 迁移计划
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    /// 待执行的操作
    pub operations: Vec<MigrationOperation>,
}

impl MigrationPlan {
    /// 创建空计划
    pub fn empty() -> Self {
        Self { operations: Vec::new() }
    }

    /// 计划是否为空
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// 添加操作
    pub fn add_operation(&mut self, op: MigrationOperation) {
        self.operations.push(op);
    }

    /// 检测是否包含破坏性操作
    pub fn has_destructive_operations(&self) -> bool {
        self.operations.iter().any(Self::is_destructive)
    }

    /// 判断操作是否是破坏性的
    fn is_destructive(op: &MigrationOperation) -> bool {
        matches!(
            op,
            MigrationOperation::DropTable { .. }
                | MigrationOperation::DropColumn { .. }
                | MigrationOperation::AlterColumn { .. }
        )
    }

    /// 获取所有破坏性操作
    pub fn destructive_operations(&self) -> Vec<&MigrationOperation> {
        self.operations
            .iter()
            .filter(|op| Self::is_destructive(op))
            .collect()
    }

    /// 生成 SQL 语句列表
    pub fn to_sql(&self, db_type: DatabaseType) -> Vec<String> {
        let mut sqls = Vec::new();
        for op in &self.operations {
            match op {
                MigrationOperation::CreateTable { schema } => {
                    sqls.extend(schema.to_full_create_sql_for(db_type));
                }
                MigrationOperation::DropTable { table_name } => {
                    sqls.push(format!("DROP TABLE IF EXISTS {}", table_name));
                }
                MigrationOperation::AddColumn { table_name, column } => {
                    sqls.push(format!(
                        "ALTER TABLE {} ADD COLUMN {}",
                        table_name,
                        column.to_sql_for(db_type)
                    ));
                }
                MigrationOperation::DropColumn { table_name, column_name } => {
                    sqls.push(format!("ALTER TABLE {} DROP COLUMN {}", table_name, column_name));
                }
                MigrationOperation::AlterColumn { table_name, column } => {
                    sqls.push(format!(
                        "ALTER TABLE {} ALTER COLUMN {}",
                        table_name,
                        column.to_sql_for(db_type)
                    ));
                }
                MigrationOperation::CreateIndex { table_name: _, index } => {
                    sqls.push(index.to_create_sql_for(db_type));
                }
                MigrationOperation::DropIndex { table_name: _, index_name } => {
                    sqls.push(format!("DROP INDEX IF EXISTS {}", index_name));
                }
            }
        }
        sqls
    }

    /// 打印迁移计划
    pub fn print(&self, db_type: DatabaseType) {
        if self.is_empty() {
            println!("No migration needed. Database is already up to date.");
            return;
        }

        println!("Migration Plan ({} operations):", self.operations.len());
        println!("{}", "=".repeat(60));

        for (i, op) in self.operations.iter().enumerate() {
            println!("\n{}. {}", i + 1, operation_description(op));
        }

        println!("\n{}", "=".repeat(60));
        println!("\nSQL Statements:");
        for sql in self.to_sql(db_type) {
            println!("  {sql};");
        }
    }
}

/// Schema 同步器
pub struct SchemaSynchronizer {
    /// 完整的 schema 配置（包含数据库配置）
    config: SchemaConfig,
}

impl SchemaSynchronizer {
    /// 从 YAML 文件创建同步器（新格式）
    pub fn from_yaml_file(path: impl AsRef<std::path::Path>) -> WaeResult<Self> {
        let config = load_schema_config_from_yaml_file(path)?;
        Ok(Self { config })
    }

    /// 从已加载的 schema config 创建同步器
    pub fn new(config: SchemaConfig) -> Self {
        Self { config }
    }

    /// 获取 schema 配置
    pub fn config(&self) -> &SchemaConfig {
        &self.config
    }

    /// 获取数据库配置
    pub fn database_config(&self) -> &DatabaseLinkConfig {
        &self.config.database
    }

    /// 获取数据库类型
    pub fn db_type(&self) -> DatabaseType {
        self.config.database.r#type
    }

    /// 获取期望的 schemas
    pub fn expected_schemas(&self) -> &[TableSchema] {
        &self.config.schemas
    }

    /// 获取数据库连接字符串
    pub fn database_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.config.database.get_url()
    }

    /// 生成迁移 SQL（预览模式）
    pub fn generate_preview_sql(&self) -> Vec<String> {
        let mut sqls = Vec::new();
        for schema in &self.config.schemas {
            sqls.extend(schema.to_full_create_sql_for(self.db_type()));
        }
        sqls
    }

    /// 打印预览 SQL
    pub fn print_preview(&self) {
        println!("Schema Preview:");
        println!("{}", "=".repeat(60));
        println!("Database type: {:?}", self.db_type());
        
        if let Ok(url) = self.database_url() {
            println!("Database URL: {}", mask_database_url(&url));
        }
        
        println!("Tables: {}", self.config.schemas.len());
        println!();

        for schema in &self.config.schemas {
            println!("  Table: {}", schema.name);
            println!("  Columns: {}", schema.columns.len());
            println!("  Indexes: {}", schema.indexes.len());
            println!();
        }

        println!("\n{}", "=".repeat(60));
        println!("\nGenerated SQL:");
        for sql in self.generate_preview_sql() {
            println!("  {sql};");
        }
    }
}

/// 屏蔽数据库 URL 中的密码
fn mask_database_url(url: &str) -> String {
    if let Some(pos) = url.find('@') {
        if let Some(protocol_pos) = url.find("://") {
            let protocol = &url[..=protocol_pos + 2];
            let rest = &url[pos + 1..];
            format!("{}***@{}", protocol, rest)
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    }
}
