use clap::Parser;
use wae_tools::cmds::{new::NewCommand, dev::DevCommand, generate::GenerateCommand, pull::PullCommand, push::PushCommand};

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
        subcommand: wae_tools::cmds::migrate::MigrateCommand,
    },
    /// 创建新项目脚手架
    New(NewCommand),
    /// 热重载开发服务器
    Dev(DevCommand),
    /// 从 OpenAPI/Swagger 生成代码
    Generate(GenerateCommand),
    /// 从远程同步 WAE 文件
    Pull(PullCommand),
    /// 推送到数据库
    Push(PushCommand),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
        Commands::Migrate { subcommand } => {
            if let Err(e) = subcommand.run().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::New(cmd) => {
            if let Err(e) = cmd.run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Dev(cmd) => {
            if let Err(e) = cmd.run().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Generate(cmd) => {
            if let Err(e) = cmd.run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Pull(cmd) => {
            if let Err(e) = cmd.run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Push(cmd) => {
            if let Err(e) = cmd.run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}