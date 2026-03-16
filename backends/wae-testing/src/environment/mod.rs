//! 测试环境管理模块
//! 
//! 提供完整的测试生命周期管理、多服务集成测试支持和容器化测试环境。

use parking_lot::RwLock;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use wae_types::{WaeError, WaeErrorKind, WaeResult as TestingResult};

mod builder;
mod config;
mod hooks;
mod state;

pub use builder::TestEnvBuilder;
pub use config::{TestEnvConfig, TestServiceConfig};
pub use hooks::{AsyncTestLifecycleHook, TestLifecycleHook};
pub use state::TestEnvState;

/// 测试环境管理器
///
/// 提供完整的测试生命周期管理，支持同步和异步钩子函数，多服务集成测试，以及容器化测试环境。
pub struct TestEnv {
    /// 配置
    config: TestEnvConfig,
    /// 状态
    state: Arc<RwLock<TestEnvState>>,
    /// 创建时间
    created_at: Instant,
    /// 初始化完成时间
    initialized_at: Arc<RwLock<Option<Instant>>>,
    /// 同步生命周期钩子
    lifecycle_hooks: Arc<RwLock<Vec<Box<dyn TestLifecycleHook>>>>,
    /// 异步生命周期钩子
    async_lifecycle_hooks: Arc<RwLock<Vec<Box<dyn AsyncTestLifecycleHook>>>>,
    /// 清理函数列表
    #[allow(clippy::type_complexity)]
    cleanup_handlers: Arc<RwLock<Vec<Box<dyn Fn() + Send + Sync>>>>,
    /// 异步清理函数列表
    #[allow(clippy::type_complexity)]
    async_cleanup_handlers: Arc<RwLock<Vec<Box<dyn Fn() -> std::pin::Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>>>,
    /// 存储的数据
    storage: Arc<RwLock<std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>>>,
    /// 测试服务配置
    services: Arc<RwLock<std::collections::HashMap<String, TestServiceConfig>>>,
}

impl std::fmt::Debug for TestEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestEnv")
            .field("config", &self.config)
            .field("state", &self.state)
            .field("created_at", &self.created_at)
            .field("initialized_at", &self.initialized_at)
            .field("services", &self.services)
            .finish()
    }
}

impl TestEnv {
    /// 创建新的测试环境
    ///
    /// # Examples
    ///
    /// ```
    /// use wae_testing::{TestEnv, TestEnvConfig};
    ///
    /// let config = TestEnvConfig::default();
    /// let env = TestEnv::new(config);
    /// ```
    pub fn new(config: TestEnvConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(TestEnvState::Uninitialized)),
            created_at: Instant::now(),
            initialized_at: Arc::new(RwLock::new(None)),
            lifecycle_hooks: Arc::new(RwLock::new(Vec::new())),
            async_lifecycle_hooks: Arc::new(RwLock::new(Vec::new())),
            cleanup_handlers: Arc::new(RwLock::new(Vec::new())),
            async_cleanup_handlers: Arc::new(RwLock::new(Vec::new())),
            storage: Arc::new(RwLock::new(std::collections::HashMap::new())),
            services: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 创建默认测试环境
    ///
    /// # Examples
    ///
    /// ```
    /// use wae_testing::TestEnv;
    ///
    /// let env = TestEnv::default_env();
    /// ```
    pub fn default_env() -> Self {
        Self::new(TestEnvConfig::default())
    }

    /// 初始化测试环境
    ///
    /// 按顺序执行所有 `before_setup` 钩子、初始化环境、然后执行所有 `after_setup` 钩子。
    ///
    /// # Errors
    ///
    /// 如果环境已初始化或任何钩子执行失败，将返回 [`WaeError`] 错误。
    ///
    /// # Examples
    ///
    /// ```
    /// use wae_testing::TestEnv;
    ///
    /// let env = TestEnv::default_env();
    /// env.setup().unwrap();
    /// ```
    pub fn setup(&self) -> TestingResult<()> {
        {
            let mut state = self.state.write();
            if *state != TestEnvState::Uninitialized {
                return Err(WaeError::new(WaeErrorKind::EnvironmentError {
                    reason: "Environment already initialized".to_string(),
                }));
            }
            *state = TestEnvState::Initializing;
        }

        let result = (|| {
            for hook in self.lifecycle_hooks.read().iter() {
                hook.before_setup(self)?;
            }

            for hook in self.lifecycle_hooks.read().iter() {
                hook.after_setup(self)?;
            }

            Ok(())
        })();

        let mut state = self.state.write();
        match result {
            Ok(_) => {
                *state = TestEnvState::Initialized;
                *self.initialized_at.write() = Some(Instant::now());
                Ok(())
            }
            Err(e) => {
                *state = TestEnvState::Uninitialized;
                Err(e)
            }
        }
    }

    /// 异步初始化测试环境
    ///
    /// 异步执行所有生命周期钩子函数，适合需要异步初始化的场景。
    ///
    /// # Errors
    ///
    /// 如果环境已初始化或任何钩子执行失败，将返回 [`WaeError`] 错误。
    pub async fn setup_async(&self) -> TestingResult<()> {
        {
            let mut state = self.state.write();
            if *state != TestEnvState::Uninitialized {
                return Err(WaeError::new(WaeErrorKind::EnvironmentError {
                    reason: "Environment already initialized".to_string(),
                }));
            }
            *state = TestEnvState::Initializing;
        }

        let result = (async {
            for hook in self.lifecycle_hooks.read().iter() {
                hook.before_setup(self)?;
            }

            #[allow(clippy::await_holding_lock)]
            for hook in self.async_lifecycle_hooks.read().iter() {
                hook.before_setup_async(self).await?;
            }

            #[allow(clippy::await_holding_lock)]
            for hook in self.async_lifecycle_hooks.read().iter() {
                hook.after_setup_async(self).await?;
            }

            for hook in self.lifecycle_hooks.read().iter() {
                hook.after_setup(self)?;
            }

            Ok(())
        })
        .await;

        let mut state = self.state.write();
        match result {
            Ok(_) => {
                *state = TestEnvState::Initialized;
                *self.initialized_at.write() = Some(Instant::now());
                Ok(())
            }
            Err(e) => {
                *state = TestEnvState::Uninitialized;
                Err(e)
            }
        }
    }

    /// 清理测试环境
    ///
    /// 按顺序执行所有 `before_teardown` 钩子、清理资源、然后执行所有 `after_teardown` 钩子。
    ///
    /// # Errors
    ///
    /// 如果环境未初始化或任何钩子执行失败，将返回 [`WaeError`] 错误。
    ///
    /// # Examples
    ///
    /// ```
    /// use wae_testing::TestEnv;
    ///
    /// let env = TestEnv::default_env();
    /// env.setup().unwrap();
    /// env.teardown().unwrap();
    /// ```
    pub fn teardown(&self) -> TestingResult<()> {
        {
            let mut state = self.state.write();
            if *state != TestEnvState::Initialized {
                return Err(WaeError::new(WaeErrorKind::EnvironmentError {
                    reason: "Environment not initialized".to_string(),
                }));
            }
            *state = TestEnvState::Destroying;
        }

        let result = (|| {
            for hook in self.lifecycle_hooks.read().iter() {
                hook.before_teardown(self)?;
            }

            let handlers = self.cleanup_handlers.write();
            for handler in handlers.iter().rev() {
                handler();
            }

            self.storage.write().clear();

            for hook in self.lifecycle_hooks.read().iter() {
                hook.after_teardown(self)?;
            }

            Ok(())
        })();

        let mut state = self.state.write();
        *state = TestEnvState::Destroyed;
        result
    }

    /// 异步清理测试环境
    ///
    /// 异步执行所有清理操作，适合需要异步清理的场景。
    ///
    /// # Errors
    ///
    /// 如果环境未初始化或任何钩子执行失败，将返回 [`WaeError`] 错误。
    pub async fn teardown_async(&self) -> TestingResult<()> {
        {
            let mut state = self.state.write();
            if *state != TestEnvState::Initialized {
                return Err(WaeError::new(WaeErrorKind::EnvironmentError {
                    reason: "Environment not initialized".to_string(),
                }));
            }
            *state = TestEnvState::Destroying;
        }

        let result = (async {
            for hook in self.lifecycle_hooks.read().iter() {
                hook.before_teardown(self)?;
            }

            #[allow(clippy::await_holding_lock)]
            for hook in self.async_lifecycle_hooks.read().iter() {
                hook.before_teardown_async(self).await?;
            }

            #[allow(clippy::await_holding_lock)]
            {
                let handlers = self.async_cleanup_handlers.write();
                for handler in handlers.iter().rev() {
                    handler().await;
                }
            }

            {
                let handlers = self.cleanup_handlers.write();
                for handler in handlers.iter().rev() {
                    handler();
                }
            }

            self.storage.write().clear();

            #[allow(clippy::await_holding_lock)]
            for hook in self.async_lifecycle_hooks.read().iter() {
                hook.after_teardown_async(self).await?;
            }

            for hook in self.lifecycle_hooks.read().iter() {
                hook.after_teardown(self)?;
            }

            Ok(())
        })
        .await;

        let mut state = self.state.write();
        *state = TestEnvState::Destroyed;
        result
    }

    /// 获取环境状态
    ///
    /// # Examples
    ///
    /// ```
    /// use wae_testing::{TestEnv, TestEnvState};
    ///
    /// let env = TestEnv::default_env();
    /// assert_eq!(env.state(), TestEnvState::Uninitialized);
    /// ```
    pub fn state(&self) -> TestEnvState {
        self.state.read().clone()
    }

    /// 获取环境运行时间
    ///
    /// 返回从环境创建到现在经过的时间。
    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// 获取环境初始化后运行的时间
    ///
    /// 如果环境尚未初始化，返回 [`None`]。
    pub fn initialized_elapsed(&self) -> Option<Duration> {
        self.initialized_at.read().map(|t| t.elapsed())
    }

    /// 注册同步生命周期钩子
    ///
    /// 添加一个同步钩子函数，在测试环境的各个生命周期阶段执行。
    ///
    /// # Examples
    ///
    /// ```
    /// use wae_testing::TestEnv;
    ///
    /// struct MyHook;
    ///
    /// impl wae_testing::TestLifecycleHook for MyHook {
    ///     fn after_setup(&self, _env: &TestEnv) -> wae_testing::TestingResult<()> {
    ///         println!("Environment setup complete!");
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let env = TestEnv::default_env();
    /// env.add_lifecycle_hook(MyHook);
    /// ```
    pub fn add_lifecycle_hook<H>(&self, hook: H)
    where
        H: TestLifecycleHook + 'static,
    {
        self.lifecycle_hooks.write().push(Box::new(hook));
    }

    /// 注册异步生命周期钩子
    ///
    /// 添加一个异步钩子函数，在测试环境的各个生命周期阶段异步执行。
    pub fn add_async_lifecycle_hook<H>(&self, hook: H)
    where
        H: AsyncTestLifecycleHook + 'static,
    {
        self.async_lifecycle_hooks.write().push(Box::new(hook));
    }

    /// 注册清理函数
    ///
    /// 添加一个同步清理函数，在测试环境清理时执行。清理函数按注册的逆序执行。
    ///
    /// # Examples
    ///
    ///