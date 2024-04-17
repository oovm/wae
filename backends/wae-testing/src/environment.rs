//! 测试环境管理模块

use crate::error::{TestingError, TestingResult};
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc, time::Duration};

/// 测试环境配置
#[derive(Debug, Clone)]
pub struct TestEnvConfig {
    /// 环境名称
    pub name: String,
    /// 是否启用日志
    pub enable_logging: bool,
    /// 是否启用追踪
    pub enable_tracing: bool,
    /// 超时时间
    pub default_timeout: Duration,
    /// 自定义配置
    pub custom: HashMap<String, String>,
}

impl Default for TestEnvConfig {
    fn default() -> Self {
        Self {
            name: "test".to_string(),
            enable_logging: true,
            enable_tracing: false,
            default_timeout: Duration::from_secs(30),
            custom: HashMap::new(),
        }
    }
}

impl TestEnvConfig {
    /// 创建新的测试环境配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置环境名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 启用日志
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }

    /// 启用追踪
    pub fn with_tracing(mut self, enable: bool) -> Self {
        self.enable_tracing = enable;
        self
    }

    /// 设置超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// 添加自定义配置
    pub fn custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }
}

/// 测试环境状态
#[derive(Debug, Clone, PartialEq)]
pub enum TestEnvState {
    /// 未初始化
    Uninitialized,
    /// 已初始化
    Initialized,
    /// 已销毁
    Destroyed,
}

/// 测试环境管理器
pub struct TestEnv {
    /// 配置
    config: TestEnvConfig,
    /// 状态
    state: Arc<RwLock<TestEnvState>>,
    /// 清理函数列表
    #[allow(clippy::type_complexity)]
    cleanup_handlers: Arc<RwLock<Vec<Box<dyn Fn() + Send + Sync>>>>,
    /// 存储的数据
    storage: Arc<RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>>,
}

impl TestEnv {
    /// 创建新的测试环境
    pub fn new(config: TestEnvConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(TestEnvState::Uninitialized)),
            cleanup_handlers: Arc::new(RwLock::new(Vec::new())),
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 创建默认测试环境
    pub fn default_env() -> Self {
        Self::new(TestEnvConfig::default())
    }

    /// 初始化测试环境
    pub fn setup(&self) -> TestingResult<()> {
        let mut state = self.state.write();
        if *state != TestEnvState::Uninitialized {
            return Err(TestingError::EnvironmentError("Environment already initialized".to_string()));
        }

        *state = TestEnvState::Initialized;
        Ok(())
    }

    /// 清理测试环境
    pub fn teardown(&self) -> TestingResult<()> {
        let mut state = self.state.write();
        if *state != TestEnvState::Initialized {
            return Err(TestingError::EnvironmentError("Environment not initialized".to_string()));
        }

        let handlers = self.cleanup_handlers.write();
        for handler in handlers.iter() {
            handler();
        }

        self.storage.write().clear();

        *state = TestEnvState::Destroyed;
        Ok(())
    }

    /// 获取环境状态
    pub fn state(&self) -> TestEnvState {
        self.state.read().clone()
    }

    /// 注册清理函数
    pub fn on_cleanup<F>(&self, handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.cleanup_handlers.write().push(Box::new(handler));
    }

    /// 存储数据
    pub fn set<T: 'static + Send + Sync>(&self, key: &str, value: T) {
        self.storage.write().insert(key.to_string(), Box::new(value));
    }

    /// 获取数据
    pub fn get<T: 'static + Clone>(&self, key: &str) -> Option<T> {
        let storage = self.storage.read();
        storage.get(key).and_then(|v| v.downcast_ref::<T>().cloned())
    }

    /// 获取配置
    pub fn config(&self) -> &TestEnvConfig {
        &self.config
    }

    /// 使用 fixture 运行测试
    pub async fn with_fixture<F, R>(&self, fixture: F) -> TestingResult<R>
    where
        F: FnOnce() -> TestingResult<R>,
    {
        self.setup()?;

        let fixture_data = fixture()?;

        self.teardown()?;

        Ok(fixture_data)
    }

    /// 运行异步测试
    pub async fn run_test<F, Fut>(&self, test: F) -> TestingResult<()>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = TestingResult<()>>,
    {
        self.setup()?;

        let result = test().await;

        self.teardown()?;

        result
    }
}

/// 测试环境构建器
pub struct TestEnvBuilder {
    config: TestEnvConfig,
}

impl TestEnvBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { config: TestEnvConfig::default() }
    }

    /// 设置环境名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config.name = name.into();
        self
    }

    /// 启用日志
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.config.enable_logging = enable;
        self
    }

    /// 启用追踪
    pub fn with_tracing(mut self, enable: bool) -> Self {
        self.config.enable_tracing = enable;
        self
    }

    /// 设置超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.default_timeout = timeout;
        self
    }

    /// 添加自定义配置
    pub fn custom(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.custom.insert(key.into(), value.into());
        self
    }

    /// 构建测试环境
    pub fn build(self) -> TestEnv {
        TestEnv::new(self.config)
    }
}

impl Default for TestEnvBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 创建测试环境
pub fn create_test_env() -> TestEnv {
    TestEnv::default_env()
}

/// 使用配置创建测试环境
pub fn create_test_env_with_config(config: TestEnvConfig) -> TestEnv {
    TestEnv::new(config)
}
