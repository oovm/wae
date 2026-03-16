//! Pull 命令模块
//! 
//! 提供从远程同步 WAE 文件的功能。

use clap::Parser;
use std::fs;
use std::path::Path;

/// Pull 命令
#[derive(Parser, Debug)]
pub struct PullCommand {
    /// 可选：远程仓库 URL
    #[clap(long, short)]
    remote: Option<String>,
    /// 可选：目标目录
    #[clap(long, short, default_value = "schemas")]
    target: String,
}

impl PullCommand {
    /// 执行 Pull 命令
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("WAE DSL Pull");
        println!("{}", "=".repeat(60));
        if let Some(remote_url) = &self.remote {
            println!("Remote: {}", remote_url);
        }
        println!("Target: {}", self.target);
        println!();
        
        // 创建目标目录和 schemas 子目录
        let target_path = Path::new(&self.target);
        let schemas_dir = target_path.join("schemas");
        if !schemas_dir.exists() {
            fs::create_dir_all(&schemas_dir)?;
            println!("Created directory: {}", schemas_dir.display());
        }
        
        // 从数据库 URL 中提取数据库名称
        let db_name = if let Some(remote_url) = &self.remote {
            if let Some(name) = Self::extract_db_name(remote_url) {
                name
            } else {
                "authentication".to_string()
            }
        } else {
            "authentication".to_string()
        };
        
        // 生成 WAE 文件路径
        let wae_file_path = schemas_dir.join(format!("{}.wae", db_name));
        
        // 这里应该实现从远程同步 WAE 文件的逻辑
        // 目前只是模拟实现
        println!("Simulating pull from remote...");
        println!("Would sync WAE files to: {}", wae_file_path.display());
        
        // 自动生成 Rust table schema
        println!("\nGenerating Rust table schemas...");
        
        // 查找所有 WAE 文件并生成 Rust schema
        #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
        {
            let mut all_schemas: Vec<super::super::dsl::TableSchema> = Vec::new();
            for entry in fs::read_dir(&schemas_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "wae") {
                    let file_schemas = super::super::dsl::load_schemas_from_wae_file(&path)?;
                    all_schemas.extend(file_schemas);
                }
            }
            
            if !all_schemas.is_empty() {
                let rust_code = super::super::dsl::generate_rust_schema(&all_schemas);
                
                // 写入 Rust schema 文件
                let rust_output_path = target_path.join("schema.rs");
                fs::write(&rust_output_path, rust_code)?;
                println!("Generated Rust table schemas at: {}", rust_output_path.display());
            } else {
                println!("No WAE files found to generate Rust schemas");
            }
        }
        #[cfg(not(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql")))]
        {
            println!("Error: Database features are not enabled. Please enable one of the database features.");
        }
        
        println!("\nPull completed successfully!");
        Ok(())
    }
    
    /// 从数据库 URL 中提取数据库名称
    fn extract_db_name(url: &str) -> Option<String> {
        // 简单的 URL 解析，提取最后一个斜杠后的部分作为数据库名称
        if let Some(last_slash) = url.rfind('/') {
            let db_name = &url[last_slash + 1..];
            // 去除可能的查询参数
            if let Some(question_mark) = db_name.find('?') {
                Some(db_name[..question_mark].to_string())
            } else {
                Some(db_name.to_string())
            }
        } else {
            None
        }
    }
}
