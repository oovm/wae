//! Pull 命令模块
//! 
//! 提供从远程同步 WAE 文件的功能。

use clap::Parser;
use std::{fs, path::Path};
use wae_types::WaeResult;

/// Pull 命令
#[derive(Parser, Debug)]
pub struct PullCommand {
    /// 数据库 URL
    #[clap(long, short)]
    remote: String,
    /// 可选：目标目录
    #[clap(long, short, default_value = "schemas")]
    target: String,
}

impl PullCommand {
    /// 执行 Pull 命令
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("WAE DSL Pull");
        println!("{}", "=".repeat(60));
        println!("Remote: {}", self.remote);
        println!("Target: {}", self.target);
        println!();

        // 创建目标目录
        let schemas_dir = Path::new(&self.target);
        if !schemas_dir.exists() {
            fs::create_dir_all(&schemas_dir)?;
            println!("Created directory: {}", schemas_dir.display());
        }

        // 从数据库 URL 中提取数据库名称
        let db_name = if let Some(name) = Self::extract_db_name(&self.remote) { name } else { "authentication".to_string() };

        // 生成 WAE 文件路径
        let wae_file_path = schemas_dir.join(format!("{}.wae", db_name));

        // 实现从数据库提取 schema 的逻辑
        println!("Extracting schema from database...");
        println!("Would extract schema to: {}", wae_file_path.display());

        #[cfg(feature = "database-mysql")]
        {
            // 连接数据库
            let conn = Self::connect_to_mysql(&self.remote).await?;
            
            // 创建 SchemaReflector
            let reflector = crate::SchemaReflector::new(&conn);
            
            // 获取所有表的 schema
            let schemas = reflector.get_all_schemas().await?;
            
            // 生成 WAE 文件内容
            let wae_content = Self::generate_wae_content(&db_name, &schemas)?;
            
            // 写入 WAE 文件
            fs::write(&wae_file_path, wae_content)?;
            println!("Generated WAE file: {}", wae_file_path.display());
        }

        #[cfg(not(feature = "database-mysql"))]
        {
            // 模拟生成 WAE 文件
            println!("Simulating schema extraction...");
            let wae_content = format!("# RBQ Schema for {}\n\n@database(\"mysql.{}\")\nnamespace {};\n\n// Tables will be generated here\n", db_name, db_name, db_name);
            
            // 写入 WAE 文件
            fs::write(&wae_file_path, wae_content)?;
            println!("Generated WAE file: {}", wae_file_path.display());
        }

        println!("\nPull completed successfully!");
        Ok(())
    }

    /// 连接到 MySQL 数据库
    #[cfg(feature = "database-mysql")]
    async fn connect_to_mysql(url: &str) -> Result<wae_database::MySqlConnection, Box<dyn std::error::Error>> {
        use wae_database::{DatabaseConfig, MySqlDatabaseService};
        println!("Attempting to connect to MySQL database: {}", url);
        let config = DatabaseConfig::MySql {
            connection_string: url.to_string(),
            pool_config: Default::default(),
        };
        println!("Creating database service...");
        let service = MySqlDatabaseService::new(&config).await?;
        println!("Service created successfully. Connecting to database...");
        let conn = service.connect().await?;
        println!("Connection established successfully!");
        Ok(conn)
    }

    /// 生成 WAE 文件内容
    #[cfg(feature = "database-mysql")]
    fn generate_wae_content(db_name: &str, schemas: &std::collections::HashMap<String, wae_database::TableSchema>) -> WaeResult<String> {
        let mut content = format!("# RBQ Schema for {}\n\n@database(\"mysql.{}\")\nnamespace {};\n\n", db_name, db_name, db_name);
        
        // 生成每个表的定义
        for (table_name, table_schema) in schemas {
            content.push_str(&format!("class {} {{", table_name));
            
            // 生成列定义
            for column in &table_schema.columns {
                let mut column_def = format!("    {} {},", column.name, Self::column_type_to_rbq(&column.col_type));
                
                if column.primary_key {
                    column_def.push_str(" primary_key");
                }
                
                if column.auto_increment {
                    column_def.push_str(" auto_increment");
                }
                
                if column.nullable {
                    column_def.push_str(" T");
                }
                
                if column.unique {
                    column_def.push_str(" unique");
                }
                
                if let Some(default) = &column.default_value {
                    column_def.push_str(&format!(" default '{}'", default));
                }
                
                column_def.push_str(";\n");
                content.push_str(&column_def);
            }
            
            // 生成外键定义
            for fk in &table_schema.foreign_keys {
                content.push_str(&format!("    foreign_key {} references {}({});\n", fk.column, fk.ref_table, fk.ref_column));
            }
            
            // 生成索引定义
            for index in &table_schema.indexes {
                if !index.name.starts_with("PRIMARY") {
                    let unique = if index.unique { "unique " } else { "" };
                    content.push_str(&format!("    {}index {}({});\n", unique, index.name, index.columns.join(", ")));
                }
            }
            
            content.push_str("}\n\n");
        }
        
        Ok(content)
    }

    /// 将 ColumnType 转换为 RBQ 类型
    #[cfg(feature = "database-mysql")]
    fn column_type_to_rbq(col_type: &wae_database::ColumnType) -> String {
        match col_type {
            wae_database::ColumnType::Integer => "int".to_string(),
            wae_database::ColumnType::Real => "float".to_string(),
            wae_database::ColumnType::Text => "string".to_string(),
            wae_database::ColumnType::Blob => "binary".to_string(),
        }
    }

    /// 从数据库 URL 中提取数据库名称
    fn extract_db_name(url: &str) -> Option<String> {
        // 简单的 URL 解析，提取最后一个斜杠后的部分作为数据库名称
        if let Some(last_slash) = url.rfind('/') {
            let db_name = &url[last_slash + 1..];
            // 去除可能的查询参数
            if let Some(question_mark) = db_name.find('?') {
                Some(db_name[..question_mark].to_string())
            }
            else {
                Some(db_name.to_string())
            }
        }
        else {
            None
        }
    }
}
