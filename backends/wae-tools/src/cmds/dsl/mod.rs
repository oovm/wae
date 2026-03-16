//! DSL 命令模块
//! 
//! 提供将 .wae 文件转换为 .yaml 文件的功能。

use clap::Parser;
use std::path::Path;

/// DSL 命令
#[derive(Parser, Debug)]
pub struct DslCommand {
    /// 输入的 .wae 文件路径
    #[clap(value_parser)]
    input: String,
    
    /// 输出的 .yaml 文件路径
    #[clap(value_parser)]
    output: String,
}

impl DslCommand {
    /// 执行 DSL 命令
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let input_path = Path::new(&self.input);
        let output_path = Path::new(&self.output);
        
        // 加载 .wae 文件并解析为 TableSchema
        let schemas = super::super::dsl::load_schemas_from_wae_file(input_path)?;
        
        // 将 TableSchema 导出为 YAML 文件
        let yaml = serde_yaml::to_string(&schemas)?;
        std::fs::write(output_path, yaml)?;
        
        println!("Successfully converted {} to {}", self.input, self.output);
        Ok(())
    }
}