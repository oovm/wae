use clap::Parser;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::process::Command;

/// WAE Tools CLI - 项目脚手架、代码生成、数据库迁移命令行工具
#[derive(Parser)]
#[command(name = "wae-tools", about = "WAE Tools CLI", long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// 数据库迁移相关命令
    #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
    Migrate {
        #[command(subcommand)]
        migrate_command: MigrateCommands,
    },
    /// 创建新项目脚手架
    New {
        /// 项目名称
        name: String,
        /// 可选：项目目录
        #[arg(long, short)]
        path: Option<String>,
    },
    /// 热重载开发服务器
    Dev {
        /// 可选：监听目录
        #[arg(long, short)]
        watch: Option<String>,
        /// 可选：命令
        #[arg(long, short)]
        cmd: Option<String>,
    },
    /// 从 OpenAPI/Swagger 生成代码
    Generate {
        /// OpenAPI/Swagger 文件路径或 URL
        #[arg(long, short)]
        spec: String,
        /// 输出目录
        #[arg(long, short)]
        out: Option<String>,
    },
}

#[derive(clap::Subcommand)]
#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
enum MigrateCommands {
    /// 运行所有待迁移
    Up {
        /// 可选：数据库连接字符串
        #[arg(long, short)]
        database: Option<String>,
        /// 可选：迁移目录
        #[arg(long, short)]
        migrations: Option<String>,
    },
    /// 回滚最后一次迁移
    Down {
        /// 可选：数据库连接字符串
        #[arg(long, short)]
        database: Option<String>,
        /// 可选：迁移目录
        #[arg(long, short)]
        migrations: Option<String>,
    },
    /// 查看迁移状态
    Status {
        /// 可选：数据库连接字符串
        #[arg(long, short)]
        database: Option<String>,
        /// 可选：迁移目录
        #[arg(long, short)]
        migrations: Option<String>,
    },
    /// 创建新迁移文件
    Create {
        /// 迁移名称
        name: String,
        /// 可选：迁移目录
        #[arg(long, short)]
        migrations: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
        Commands::Migrate { migrate_command } => {
            handle_migrate_command(migrate_command).await;
        }
        Commands::New { name, path } => {
            if let Err(e) = handle_new_project(name, path) {
                eprintln!("Error creating project: {}", e);
            }
        }
        Commands::Dev { watch, cmd } => {
            if let Err(e) = handle_dev(watch, cmd).await {
                eprintln!("Error starting dev server: {}", e);
            }
        }
        Commands::Generate { spec, out } => {
            if let Err(e) = handle_generate(spec, out) {
                eprintln!("Error generating code: {}", e);
            }
        }
    }
}

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
async fn handle_migrate_command(_cmd: &MigrateCommands) {
    println!("Handling migrate command... (integration with migration module coming soon)");
}

fn handle_new_project(name: &str, path: &Option<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let project_path = match path {
        Some(p) => PathBuf::from(p).join(name),
        None => PathBuf::from(name),
    };

    if project_path.exists() {
        return Err(format!("Project directory already exists: {}", project_path.display()).into());
    }

    fs::create_dir_all(&project_path)?;
    fs::create_dir_all(project_path.join("src"))?;

    let cargo_toml_content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
wae-server = {{ workspace = true }}
tokio = {{ workspace = true }}
"#,
        name
    );
    fs::write(project_path.join("Cargo.toml"), cargo_toml_content)?;

    let main_rs_content = r#"
use wae_server::prelude::*;

#[tokio::main]
async fn main() {
    println!("Hello, WAE!");
}
"#;
    fs::write(project_path.join("src/main.rs"), main_rs_content)?;

    println!("Project created successfully at: {}", project_path.display());
    Ok(())
}

async fn handle_dev(watch: &Option<String>, cmd: &Option<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let watch_path = watch.as_deref().unwrap_or(".");
    let command = cmd.as_deref().unwrap_or("cargo run");

    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(500), tx)?;
    debouncer.watcher().watch(Path::new(watch_path), RecursiveMode::Recursive)?;

    println!("Watching for changes in: {}", watch_path);
    println!("Running command: {}", command);

    let mut child = run_command(command).await?;

    println!("\nPress Ctrl+C to stop.");

    for res in rx {
        match res {
            Ok(_) => {
                println!("Changes detected, restarting...");
                if let Some(ref mut c) = child {
                    let _ = c.kill().await;
                }
                child = run_command(command).await?;
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
            }
        }
    }

    Ok(())
}

async fn run_command(cmd: &str) -> std::result::Result<Option<tokio::process::Child>, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(None);
    }

    let child = Command::new(parts[0]).args(&parts[1..]).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn()?;

    Ok(Some(child))
}

fn handle_generate(spec: &str, out: &Option<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let out_path = out.as_deref().unwrap_or("src/generated");
    println!("Generating code from spec: {}", spec);
    println!("Output directory: {}", out_path);
    println!("(OpenAPI/Swagger code generation coming soon)");
    Ok(())
}
