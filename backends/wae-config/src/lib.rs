//! WAE Config - 配置管理模块
//! 
//! 提供统一的配置加载和管理能力，基于 itools-config 库实现。
//! 支持多层级配置合并：环境变量 > 配置文件 > 默认值。

#![warn(missing_docs)]

use itools_config::ConfigManager;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;
use wae_types::{WaeError, WaeResult};

/// 配置加载器
/// 
/// 支持从多种来源加载配置，并按优先级合并。
/// 优先级从高到低：环境变量 > 指定配置文件 > 默认配置文件 > 默认值
pub struct ConfigLoader {
    config_manager: ConfigManager,
    env_prefix: Option<String>,
    env_separator: Option<String>,
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigLoader {
    /// 创建新的配置加载器
    pub fn new() -> Self {
        Self {
            config_manager: ConfigManager::new(),
            env_prefix: None,
            env_separator: None,
        }
    }

    /// 从 TOML 文件加载配置
    /// 
    /// # Arguments
    /// * `path` - 配置文件路径
    pub fn with_toml(mut self, path: &str) -> Self {
        if let Err(e) = self.config_manager.load_fragment(Path::new(path)) {
            // 忽略错误，保持与 figment 行为一致
            // figment 会在 extract 时报告错误
        }
        self
    }

    /// 从 YAML 文件加载配置
    /// 
    /// # Arguments
    /// * `path` - 配置文件路径
    pub fn with_yaml(mut self, path: &str) -> Self {
        if let Err(e) = self.config_manager.load_fragment(Path::new(path)) {
            // 忽略错误，保持与 figment 行为一致
        }
        self
    }

    /// 从环境变量加载配置
    /// 
    /// # Arguments
    /// * `prefix` - 环境变量前缀，如 "APP_"
    pub fn with_env(mut self, prefix: &str) -> Self {
        self.env_prefix = Some(prefix.to_string());
        self
    }

    /// 从环境变量加载配置（带分隔符）
    /// 
    /// # Arguments
    /// * `prefix` - 环境变量前缀
    /// * `separator` - 嵌套分隔符，如 "__"
    pub fn with_env_separator(mut self, prefix: &str, separator: &str) -> Self {
        self.env_prefix = Some(prefix.to_string());
        self.env_separator = Some(separator.to_string());
        self
    }

    /// 合并默认值
    /// 
    /// # Arguments
    /// * `defaults` - 默认配置值
    pub fn with_defaults<T: Serialize>(mut self, defaults: &T) -> Self {
        // 将默认值序列化为 JSON，然后加载为配置片段
        if let Ok(defaults_json) = serde_json::to_value(defaults) {
            // 使用 add_defaults 方法添加默认值
            self.config_manager.add_defaults(defaults_json);
        }
        self
    }

    /// 提取配置到指定类型
    /// 
    /// # Type Parameters
    /// * `T` - 配置类型，需实现 `DeserializeOwned`
    pub fn extract<T: DeserializeOwned>(mut self) -> WaeResult<T> {
        // 应用环境变量覆盖
        if let Some(prefix) = &self.env_prefix {
            // 模拟 figment 的环境变量处理
            self.apply_env_vars(prefix, self.env_separator.as_deref());
        }

        // 从合并后的配置中提取
        let config = self.config_manager.get_config();
        serde_json::from_value(config.clone()).map_err(|e| {
            let msg = e.to_string();
            if msg.contains("missing field") {
                WaeError::config_missing(msg)
            } else {
                WaeError::config_invalid("config", msg)
            }
        })
    }

    /// 提取配置到指定类型（带自定义错误消息）
    /// 
    /// # Type Parameters
    /// * `T` - 配置类型，需实现 `DeserializeOwned`
    /// * `context` - 错误上下文信息
    pub fn extract_with_context<T: DeserializeOwned>(self, context: &str) -> WaeResult<T> {
        self.extract().map_err(|e| WaeError::internal(format!("{}: {}", context, e)))
    }

    /// 应用环境变量覆盖
    fn apply_env_vars(&mut self, prefix: &str, separator: Option<&str>) {
        // 简单实现：支持前缀的环境变量
        for (key, value) in std::env::vars() {
            if key.starts_with(prefix) {
                let key_path = key.trim_start_matches(prefix);
                let key_path = if let Some(sep) = separator {
                    key_path.replace(sep, ".").to_lowercase()
                } else {
                    key_path.to_lowercase()
                };
                self.config_manager.set_config_value(&key_path, &value);
            }
        }
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
pub fn load_config<T: DeserializeOwned>(config_path: &str, env_prefix: &str) -> WaeResult<T> {
    ConfigLoader::new().with_toml(config_path).with_env(env_prefix).extract()
}

/// 从环境变量加载配置
/// 
/// # Type Parameters
/// * `T` - 配置类型
/// 
/// # Arguments
/// * `prefix` - 环境变量前缀
pub fn from_env<T: DeserializeOwned>(prefix: &str) -> WaeResult<T> {
    ConfigLoader::new().with_env(prefix).extract()
}
