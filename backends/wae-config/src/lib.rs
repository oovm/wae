//! WAE Config - 配置管理模块
//!
//! 提供统一的配置加载和管理能力，基于 figment 库实现。
//! 支持多层级配置合并：环境变量 > 配置文件 > 默认值。

#![warn(missing_docs)]

use figment::{
    Figment,
    providers::{Env, Format, Toml, Yaml},
};
use serde::de::DeserializeOwned;
use std::fmt;

/// 配置错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 配置加载失败
    LoadFailed(String),

    /// 配置解析失败
    ParseFailed(String),

    /// 配置验证失败
    ValidationFailed(String),

    /// 缺少必需配置
    MissingConfig(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::LoadFailed(msg) => write!(f, "Failed to load config: {}", msg),
            ConfigError::ParseFailed(msg) => write!(f, "Failed to parse config: {}", msg),
            ConfigError::ValidationFailed(msg) => write!(f, "Config validation failed: {}", msg),
            ConfigError::MissingConfig(msg) => write!(f, "Missing required config: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// 配置加载结果类型
pub type ConfigResult<T> = Result<T, ConfigError>;

/// 配置加载器
///
/// 支持从多种来源加载配置，并按优先级合并。
/// 优先级从高到低：环境变量 > 指定配置文件 > 默认配置文件 > 默认值
pub struct ConfigLoader {
    figment: Figment,
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigLoader {
    /// 创建新的配置加载器
    pub fn new() -> Self {
        Self { figment: Figment::new() }
    }

    /// 从 TOML 文件加载配置
    ///
    /// # Arguments
    /// * `path` - 配置文件路径
    pub fn with_toml(mut self, path: &str) -> Self {
        self.figment = self.figment.merge(Toml::file(path));
        self
    }

    /// 从 YAML 文件加载配置
    ///
    /// # Arguments
    /// * `path` - 配置文件路径
    pub fn with_yaml(mut self, path: &str) -> Self {
        self.figment = self.figment.merge(Yaml::file(path));
        self
    }

    /// 从环境变量加载配置
    ///
    /// # Arguments
    /// * `prefix` - 环境变量前缀，如 "APP_"
    pub fn with_env(mut self, prefix: &str) -> Self {
        self.figment = self.figment.merge(Env::prefixed(prefix));
        self
    }

    /// 从环境变量加载配置（带分隔符）
    ///
    /// # Arguments
    /// * `prefix` - 环境变量前缀
    /// * `separator` - 嵌套分隔符，如 "__"
    pub fn with_env_separator(mut self, prefix: &str, separator: &str) -> Self {
        self.figment = self.figment.merge(Env::prefixed(prefix).split(separator));
        self
    }

    /// 合并默认值
    ///
    /// # Arguments
    /// * `defaults` - 默认配置值
    pub fn with_defaults<T: serde::Serialize>(mut self, defaults: &T) -> Self {
        self.figment = self.figment.merge(figment::providers::Serialized::defaults(defaults));
        self
    }

    /// 提取配置到指定类型
    ///
    /// # Type Parameters
    /// * `T` - 配置类型，需实现 `DeserializeOwned`
    pub fn extract<T: DeserializeOwned>(self) -> ConfigResult<T> {
        self.figment.extract().map_err(|e| {
            let msg = e.to_string();
            if msg.contains("missing field") { ConfigError::MissingConfig(msg) } else { ConfigError::ParseFailed(msg) }
        })
    }

    /// 提取配置到指定类型（带自定义错误消息）
    ///
    /// # Type Parameters
    /// * `T` - 配置类型，需实现 `DeserializeOwned`
    /// * `context` - 错误上下文信息
    pub fn extract_with_context<T: DeserializeOwned>(self, context: &str) -> ConfigResult<T> {
        self.extract().map_err(|e| ConfigError::LoadFailed(format!("{}: {}", context, e)))
    }
}

/// 快速加载配置的便捷函数
///
/// 从默认位置加载配置文件，并合并环境变量。
///
/// # Type Parameters
/// * `T` - 配置类型
///
/// # Arguments
/// * `config_path` - 配置文件路径
/// * `env_prefix` - 环境变量前缀
pub fn load_config<T: DeserializeOwned>(config_path: &str, env_prefix: &str) -> ConfigResult<T> {
    ConfigLoader::new().with_toml(config_path).with_env(env_prefix).extract()
}

/// 从环境变量加载配置
///
/// # Type Parameters
/// * `T` - 配置类型
///
/// # Arguments
/// * `prefix` - 环境变量前缀
pub fn from_env<T: DeserializeOwned>(prefix: &str) -> ConfigResult<T> {
    ConfigLoader::new().with_env(prefix).extract()
}
