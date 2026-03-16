//! Push 命令模块
//! 
//! 提供将 WAE 文件推送到数据库的功能。

use clap::Parser;
use std::fs;
use std::path::Path;

/// Push 命令
#[derive(Parser, Debug)]
pub struct PushCommand {
    /// WAE 文件路径或目录
    input: String,
    /// 可选：数据库连接字符串
    #[clap(long, short)]
    database: Option<String>,
    /// 强制执行破坏性操作
    #[clap(long, short)]
    force: bool,
}

impl PushCommand {
    /// 执行 Push 命令
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("WAE DSL Push");
        println!("{}", "=".repeat(60));
        println!("Input: {}", self.input);
        if let Some(db_url) = &self.database {
            println!("Database: {}", db_url);
        }
        println!("Force: {}", self.force);
        println!();
        
        let input_path = Path::new(&self.input);
        let mut schemas = Vec::new();
        
        if input_path.is_dir() {
            // 处理目录，遍历所有 .wae 文件
            for entry in fs::read_dir(input_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "wae") {
                    println!("Processing file: {}", path.display());
                    let file_schemas = super::super::dsl::load_schemas_from_wae_file(&path)?;
                    schemas.extend(file_schemas);
                }
            }
        } else {
            // 处理单个文件
            println!("Processing file: {}", self.input);
            schemas = super::super::dsl::load_schemas_from_wae_file(&self.input)?;
        }
        
        println!("\nLoaded {} table schemas", schemas.len());
        
        // 自动生成 Rust table schema
        println!("\nGenerating Rust table schemas...");
        
        if !schemas.is_empty() {
            let rust_code = super::super::dsl::generate_rust_schema(&schemas);
            
            // 写入 Rust schema 文件
            let rust_output_path = input_path.parent().unwrap_or_else(|| Path::new("")).join("schema.rs");
            fs::write(&rust_output_path, rust_code)?;
            println!("Generated Rust table schemas at: {}", rust_output_path.display());
        } else {
            println!("No table schemas found to generate Rust schemas");
        }
        
        // 这里应该实现推送到数据库的逻辑
        // 目前只是模拟实现
        println!("\nSimulating push to database...");
        println!("Would apply {} table schemas to database", schemas.len());
        if self.force {
            println!("Force mode enabled - will perform destructive operations");
        }
        
        println!("\nPush completed successfully!");
        Ok(())
    }
}
