//! 测试容器管理模块
//! 
//! 提供基于 Docker 的测试容器支持，包括 PostgreSQL、MySQL 和 Redis。

#[cfg(feature = "containers")]
mod containers_impl {
    use std::process::Command;
    use testcontainers::{
        core::{ContainerAsync, ContainerPort, Image, WaitFor},
        runners::AsyncRunner,
    };

    /// 测试容器 trait，定义容器生命周期管理方法
    pub trait TestContainer {
        /// 获取容器的连接 URL
        fn connection_url(&self) -> String;

        /// 启动容器
        async fn start() -> Self
        where
            Self: Sized;

        /// 停止容器
        async fn stop(&mut self);

        /// 清理容器资源
        async fn cleanup(self);
    }

    /// 检查 Docker 是否可用
    pub fn is_docker_available() -> bool {
        let output = Command::new("docker")
            .arg("--version")
            .output();
        
        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    /// PostgreSQL 容器
    pub struct PostgresContainer {
        container: ContainerAsync<PostgresImage>,
        host: String,
        port: u16,
        username: String,
        password: String,
        database: String,
    }

    /// PostgreSQL 镜像配置
    #[derive(Debug, Clone)]
    pub struct PostgresImage {
        tag: String,
        username: String,
        password: String,
        database: String,
    }

    impl Default for PostgresImage {
        fn default() -> Self {
            Self {
                tag: "15-alpine".to_string(),
                username: "postgres".to_string(),
                password: "password".to_string(),
                database: "test_db".to_string(),
            }
        }
    }

    impl Image for PostgresImage {
        fn name(&self) -> &str {
            "postgres"
        }

        fn tag(&self) -> &str {
            &self.tag
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![
                WaitFor::message_on_stderr("database system is ready to accept connections"),
                WaitFor::millis(500),
            ]
        }

        fn env_vars(&self) -> impl IntoIterator<Item = (impl Into<std::borrow::Cow<'_, str>>, impl Into<std::borrow::Cow<'_, str>>)> {
            vec![
                ("POSTGRES_USER", self.username.clone()),
                ("POSTGRES_PASSWORD", self.password.clone()),
                ("POSTGRES_DB", self.database.clone()),
            ]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[ContainerPort::Tcp(5432)]
        }
    }

    impl PostgresContainer {
        /// 创建默认 PostgreSQL 容器
        pub async fn default() -> Self {
            Self::new(PostgresImage::default()).await
        }

        /// 使用自定义配置创建 PostgreSQL 容器
        pub async fn new(image: PostgresImage) -> Self {
            let container = image.clone().start().await.unwrap();
            let host = container.get_host().await.unwrap().to_string();
            let port = container.get_host_port_ipv4(5432).await.unwrap();

            Self {
                container,
                host,
                port,
                username: image.username,
                password: image.password,
                database: image.database,
            }
        }

        /// 获取容器主机
        pub fn host(&self) -> &str {
            &self.host
        }

        /// 获取容器端口
        pub fn port(&self) -> u16 {
            self.port
        }

        /// 获取用户名
        pub fn username(&self) -> &str {
            &self.username
        }

        /// 获取密码
        pub fn password(&self) -> &str {
            &self.password
        }

        /// 获取数据库名
        pub fn database(&self) -> &str {
            &self.database
        }
    }

    impl TestContainer for PostgresContainer {
        fn connection_url(&self) -> String {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            )
        }

        async fn start() -> Self {
            Self::default().await
        }

        async fn stop(&mut self) {
            self.container.stop().await.unwrap();
        }

        async fn cleanup(self) {
            self.container.rm().await.unwrap();
        }
    }

    /// MySQL 容器
    pub struct MySqlContainer {
        container: ContainerAsync<MySqlImage>,
        host: String,
        port: u16,
        username: String,
        password: String,
        database: String,
    }

    /// MySQL 镜像配置
    #[derive(Debug, Clone)]
    pub struct MySqlImage {
        tag: String,
        username: String,
        password: String,
        database: String,
        root_password: String,
    }

    impl Default for MySqlImage {
        fn default() -> Self {
            Self {
                tag: "8.0".to_string(),
                username: "mysql".to_string(),
                password: "password".to_string(),
                database: "test_db".to_string(),
                root_password: "root_password".to_string(),
            }
        }
    }

    impl Image for MySqlImage {
        fn name(&self) -> &str {
            "mysql"
        }

        fn tag(&self) -> &str {
            &self.tag
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![
                WaitFor::message_on_stderr("ready for connections"),
                WaitFor::millis(1000),
            ]
        }

        fn env_vars(&self) -> impl IntoIterator<Item = (impl Into<std::borrow::Cow<'_, str>>, impl Into<std::borrow::Cow<'_, str>>)> {
            vec![
                ("MYSQL_ROOT_PASSWORD", self.root_password.clone()),
                ("MYSQL_USER", self.username.clone()),
                ("MYSQL_PASSWORD", self.password.clone()),
                ("MYSQL_DATABASE", self.database.clone()),
            ]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[ContainerPort::Tcp(3306)]
        }
    }

    impl MySqlContainer {
        /// 创建默认 MySQL 容器
        pub async fn default() -> Self {
            Self::new(MySqlImage::default()).await
        }

        /// 使用自定义配置创建 MySQL 容器
        pub async fn new(image: MySqlImage) -> Self {
            let container = image.clone().start().await.unwrap();
            let host = container.get_host().await.unwrap().to_string();
            let port = container.get_host_port_ipv4(3306).await.unwrap();

            Self {
                container,
                host,
                port,
                username: image.username,
                password: image.password,
                database: image.database,
            }
        }

        /// 获取容器主机
        pub fn host(&self) -> &str {
            &self.host
        }

        /// 获取容器端口
        pub fn port(&self) -> u16 {
            self.port
        }

        /// 获取用户名
        pub fn username(&self) -> &str {
            &self.username
        }

        /// 获取密码
        pub fn password(&self) -> &str {
            &self.password
        }

        /// 获取数据库名
        pub fn database(&self) -> &str {
            &self.database
        }
    }

    impl TestContainer for MySqlContainer {
        fn connection_url(&self) -> String {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            )
        }

        async fn start() -> Self {
            Self::default().await
        }

        async fn stop(&mut self) {
            self.container.stop().await.unwrap();
        }

        async fn cleanup(self) {
            self.container.rm().await.unwrap();
        }
    }

    /// Redis 容器
    pub struct RedisContainer {
        container: ContainerAsync<RedisImage>,
        host: String,
        port: u16,
        password: Option<String>,
    }

    /// Redis 镜像配置
    #[derive(Debug, Clone)]
    pub struct RedisImage {
        tag: String,
        password: Option<String>,
    }

    impl Default for RedisImage {
        fn default() -> Self {
            Self {
                tag: "7-alpine".to_string(),
                password: None,
            }
        }
    }

    impl Image for RedisImage {
        fn name(&self) -> &str {
            "redis"
        }

        fn tag(&self) -> &str {
            &self.tag
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![
                WaitFor::message_on_stdout("Ready to accept connections"),
                WaitFor::millis(500),
            ]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[ContainerPort::Tcp(6379)]
        }
    }

    impl RedisContainer {
        /// 创建默认 Redis 容器
        pub async fn default() -> Self {
            Self::new(RedisImage::default()).await
        }

        /// 使用自定义配置创建 Redis 容器
        pub async fn new(image: RedisImage) -> Self {
            let container = image.clone().start().await.unwrap();
            let host = container.get_host().await.unwrap().to_string();
            let port = container.get_host_port_ipv4(6379).await.unwrap();

            Self {
                container,
                host,
                port,
                password: image.password,
            }
        }

        /// 获取容器主机
        pub fn host(&self) -> &str {
            &self.host
        }

        /// 获取容器端口
        pub fn port(&self) -> u16 {
            self.port
        }

        /// 获取密码（如果设置）
        pub fn password(&self) -> Option<&str> {
            self.password.as_deref()
        }
    }

    impl TestContainer for RedisContainer {
        fn connection_url(&self) -> String {
            match &self.password {
                Some(pass) => format!("redis://:{}@{}:{}", pass, self.host, self.port),
                None => format!("redis://{}:{}", self.host, self.port),
            }
        }

        async fn start() -> Self {
            Self::default().await
        }

        async fn stop(&mut self) {
            self.container.stop().await.unwrap();
        }

        async fn cleanup(self) {
            self.container.rm().await.unwrap();
        }
    }
}

#[cfg(feature = "containers")]
pub use containers_impl::*;
