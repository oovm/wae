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
        
        // 创建目标目录
        let target_path = Path::new(&self.target);
        if !target_path.exists() {
            fs::create_dir_all(target_path)?;
            println!("Created directory: {}", self.target);
        }
        
        // 这里应该实现从远程同步 WAE 文件的逻辑
        // 目前只是模拟实现
        println!("Simulating pull from remote...");
        println!("Would sync WAE files to: {}", self.target);
        
        // 自动生成 Rust table schema
        println!("\nGenerating Rust table schemas...");
        
        // 查找所有 WAE 文件并生成 Rust schema
        let mut all_schemas = Vec::new();
        for entry in fs::read_dir(target_path)? {
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
        
        println!("\nPull completed successfully!");
        Ok(())
    }
}
