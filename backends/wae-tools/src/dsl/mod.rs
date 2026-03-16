//! DSL 解析模块
//! 
//! 提供对 .wae 文件的解析功能，将 DSL 定义转换为数据库 schema。

use std::fs;
use std::path::Path;
use yyds_gateway::rbq::{RbqParser, RbqStatement};
use wae_database::{ColumnDef, ColumnType, DatabaseSchema, TableSchema, IndexDef, ForeignKeyDef, ReferentialAction};

/// 从 .wae 文件加载并解析 TableSchema
pub fn load_schemas_from_wae_file(path: impl AsRef<Path>) -> Result<Vec<TableSchema>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    load_schemas_from_wae(&content)
}

/// 从 .wae 字符串加载并解析 TableSchema
pub fn load_schemas_from_wae(wae_str: &str) -> Result<Vec<TableSchema>, Box<dyn std::error::Error>> {
    let parser = RbqParser::new();
    let statements = parser.parse(wae_str)?;
    
    let mut schemas = Vec::new();
    for statement in statements {
        if let Some(schema) = parse_rbq_statement(&statement) {
            schemas.push(schema);
        }
    }
    
    Ok(schemas)
}

/// 解析 RBQ 语句为 TableSchema
fn parse_rbq_statement(statement: &RbqStatement) -> Option<TableSchema> {
    // 这里需要根据实际的 RbqStatement 结构进行解析
    // 暂时返回一个示例实现
    Some(TableSchema::new("example"))
}

/// 将 RBQ 类型转换为 ColumnType
fn convert_rbq_type_to_column_type(rbq_type: &str) -> ColumnType {
    match rbq_type.to_lowercase().as_str() {
        "i32" | "i64" | "int" | "integer" => ColumnType::Integer,
        "f32" | "f64" | "float" | "real" => ColumnType::Real,
        "string" | "text" | "varchar" => ColumnType::Text,
        "bytes" | "blob" => ColumnType::Blob,
        _ => ColumnType::Text,
    }
}