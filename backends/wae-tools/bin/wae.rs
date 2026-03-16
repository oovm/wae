use clap::Parser;
use wae_tools::cmds::{self, dsl::DslCommand, migrate::MigrateCommand, new::NewCommand, dev::DevCommand, generate::GenerateCommand};

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
        migrate_command: MigrateCommand,
    },
    /// DSL 相关命令
    #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
    Dsl {
        #[command(subcommand)]
        dsl_command: DslCommand,
    },
    /// 创建新项目脚手架
    New(NewCommand),
    /// 热重载开发服务器
    Dev(DevCommand),
    /// 从 OpenAPI/Swagger 生成代码
    Generate(GenerateCommand),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
        Commands::Migrate { migrate_command } => {
            if let Err(e) = migrate_command.run().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
        Commands::Dsl { dsl_command } => {
            if let Err(e) = dsl_command.run() {
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
    }
}
