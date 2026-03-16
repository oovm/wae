//! DSL 命令模块
//! 
//! 提供将 .wae 文件转换为 .yaml 文件的功能。

use clap::Parser;
use std::path::Path;

/// DSL 命令
#[derive(Parser, Debug)]
pub enum DslCommand {
    /// 将 .wae 文件转换为 .yaml 文件
    Convert {
        /// 输入的 .wae 文件路径
        input: String,
        /// 输出的 .yaml 文件路径
        output: String,
    },
    /// 从远程同步 WAE 文件
    Pull {
        /// 可选：远程仓库 URL
        #[clap(long, short)]
        remote: Option<String>,
        /// 可选：目标目录
        #[clap(long, short, default_value = "schemas")]
        target: String,
    },
    /// 推送到数据库
    Push {
        /// WAE 文件路径或目录
        input: String,
        /// 可选：数据库连接字符串
        #[clap(long, short)]
        database: Option<String>,
        /// 强制执行破坏性操作
        #[clap(long, short)]
        force: bool,
    },
}

impl DslCommand {
    /// 执行 DSL 命令
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            DslCommand::Convert { input, output } => {
                let input_path = Path::new(input);
                let output_path = Path::new(output);
                
                // 加载 .wae 文件并解析为 TableSchema
                let schemas = super::super::dsl::load_schemas_from_wae_file(input_path)?;
                
                // 将 TableSchema 导出为 YAML 文件
                let yaml = serde_yaml::to_string(&schemas)?;
                std::fs::write(output_path, yaml)?;
                
                println!("Successfully converted {} to {}", input, output);
                Ok(())
            }
            _ => {
                println!("Handling DSL command... (integration coming soon)");
                Ok(())
            }
        }
    }
}