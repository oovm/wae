//! 命令模块
//! 
//! 包含所有 WAE Tools 的命令定义。

pub mod migrate;
pub mod new;
pub mod dev;
pub mod generate;
pub mod pull;
pub mod push;


#[derive(clap::Subcommand)]
pub enum Commands {
    /// 数据库迁移相关命令
    #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
    Migrate(MigrateCommand),
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

// 共享参数
pub struct Arguements {

}

impl Commands {
async fn run(&self, args: &Arguements) -> WaeResult<()> {
    match self {
        #[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
        Commands::Migrate(cmd) => {
            if let Err(e) = cmd.run().await {
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
}