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
    /// DSL 相关命令
    #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
    Dsl {
        #[command(subcommand)]
        dsl_command: DslCommands,
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
enum DslCommands {
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
        #[arg(long, short)]
        remote: Option<String>,
        /// 可选：目标目录
        #[arg(long, short, default_value = "schemas")]
        target: String,
    },
    /// 推送到数据库
    Push {
        /// WAE 文件路径或目录
        input: String,
        /// 可选：数据库连接字符串
        #[arg(long, short)]
        database: Option<String>,
        /// 强制执行破坏性操作
        #[arg(long, short)]
        force: bool,
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
    /// 从 schemas.yaml 同步数据库 schema
    Sync {
        /// schemas.yaml 文件路径
        #[arg(long, short, default_value = "schemas.yaml")]
        schema: String,
        /// 是否自动执行迁移（否则仅打印计划）
        #[arg(long, short, default_value_t = false)]
        execute: bool,
        /// 是否强制执行破坏性操作（删除表、列等）
        #[arg(long, default_value_t = false)]
        force: bool,
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
        #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
        Commands::Dsl { dsl_command } => {
            handle_dsl_command(dsl_command).await;
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
async fn handle_dsl_command(cmd: &DslCommands) {
    match cmd {
        DslCommands::Convert { input, output } => {
            if let Err(e) = handle_dsl_convert(input, output) {
                eprintln!("Error converting DSL: {}", e);
                std::process::exit(1);
            }
        }
        DslCommands::Pull { remote, target } => {
            if let Err(e) = handle_dsl_pull(remote, target).await {
                eprintln!("Error pulling WAE files: {}", e);
                std::process::exit(1);
            }
        }
        DslCommands::Push { input, database, force } => {
            if let Err(e) = handle_dsl_push(input, database, *force).await {
                eprintln!("Error pushing to database: {}", e);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
async fn handle_dsl_convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    use wae_tools::dsl::load_schemas_from_wae_file;
    use serde_yaml;
    use std::fs;
    
    println!("WAE DSL Convert");
    println!("{}", "=".repeat(60));
    println!("Input: {}", input);
    println!("Output: {}", output);
    println!();
    
    // 加载 .wae 文件并解析为 TableSchema
    let schemas = load_schemas_from_wae_file(input)?;
    
    // 将 TableSchema 导出为 YAML 文件
    let yaml = serde_yaml::to_string(&schemas)?;
    fs::write(output, yaml)?;
    
    println!("Successfully converted {} to {}", input, output);
    Ok(())
}

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
async fn handle_dsl_pull(remote: &Option<String>, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    
    println!("WAE DSL Pull");
    println!("{}", "=".repeat(60));
    if let Some(remote_url) = remote {
        println!("Remote: {}", remote_url);
    }
    println!("Target: {}", target);
    println!();
    
    // 创建目标目录
    let target_path = Path::new(target);
    if !target_path.exists() {
        fs::create_dir_all(target_path)?;
        println!("Created directory: {}", target);
    }
    
    // 这里应该实现从远程同步 WAE 文件的逻辑
    // 目前只是模拟实现
    println!("Simulating pull from remote...");
    println!("Would sync WAE files to: {}", target);
    
    // 自动生成 Rust table schema
    println!("\nGenerating Rust table schemas...");
    println!("Rust table schemas would be generated based on WAE files");
    
    println!("\nPull completed successfully!");
    Ok(())
}

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
async fn handle_dsl_push(input: &str, database: &Option<String>, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    use wae_tools::dsl::load_schemas_from_wae_file;
    use std::fs;
    use std::path::Path;
    
    println!("WAE DSL Push");
    println!("{}", "=".repeat(60));
    println!("Input: {}", input);
    if let Some(db_url) = database {
        println!("Database: {}", db_url);
    }
    println!("Force: {}", force);
    println!();
    
    let input_path = Path::new(input);
    let mut schemas = Vec::new();
    
    if input_path.is_dir() {
        // 处理目录，遍历所有 .wae 文件
        for entry in fs::read_dir(input_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "wae") {
                println!("Processing file: {}", path.display());
                let file_schemas = load_schemas_from_wae_file(&path)?;
                schemas.extend(file_schemas);
            }
        }
    } else {
        // 处理单个文件
        println!("Processing file: {}", input);
        schemas = load_schemas_from_wae_file(input)?;
    }
    
    println!("\nLoaded {} table schemas", schemas.len());
    
    // 自动生成 Rust table schema
    println!("\nGenerating Rust table schemas...");
    println!("Rust table schemas would be generated based on WAE files");
    
    // 这里应该实现推送到数据库的逻辑
    // 目前只是模拟实现
    println!("\nSimulating push to database...");
    println!("Would apply {} table schemas to database", schemas.len());
    if force {
        println!("Force mode enabled - will perform destructive operations");
    }
    
    println!("\nPush completed successfully!");
    Ok(())
}

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
async fn handle_migrate_command(cmd: &MigrateCommands) {
    match cmd {
        MigrateCommands::Sync {
            schema,
            execute,
            force,
        } => {
            if let Err(e) = handle_sync_command(schema, *execute, *force).await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            println!("Handling migrate command... (integration with migration module coming soon)");
        }
    }
}

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
async fn handle_sync_command(
    schema_path: &str,
    execute: bool,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use wae_tools::schema_sync::SchemaSynchronizer;

    println!("WAE Database Schema Sync");
    println!("{}", "=".repeat(60));
    println!("Schema file: {}", schema_path);
    println!();

    let synchronizer = SchemaSynchronizer::from_yaml_file(schema_path)?;
    synchronizer.print_preview();

    if execute {
        println!("\n⚠️  Note: Full database migration execution requires additional setup.");
        println!("   Preview generation is complete. SQL can be manually applied.");
    }

    Ok(())
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
