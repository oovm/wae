//! WAE Cache - 缓存服务抽象层
//!
//! 提供统一的缓存能力抽象，支持多种缓存后端。
//!
//! 深度融合 tokio 运行时，所有 API 都是异步优先设计。
//! 微服务架构友好，支持分布式缓存、过期策略等特性。

#![warn(missing_docs)]

use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;
use wae_types::WaeError;

/// 缓存操作结果类型
pub type CacheResult<T> = Result<T, WaeError>;

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 缓存键前缀
    pub key_prefix: String,
    /// 默认过期时间
    pub default_ttl: Option<Duration>,
    /// 连接超时
    pub connection_timeout: Duration,
    /// 操作超时
    pub operation_timeout: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            key_prefix: String::new(),
            default_ttl: Some(Duration::from_secs(3600)),
            connection_timeout: Duration::from_secs(5),
            operation_timeout: Duration::from_secs(3),
        }
    }
}

/// 缓存服务核心 trait (dyn 兼容)
///
/// 定义统一的缓存操作接口，使用原始字节进行存储。
/// 所有方法都是异步的，适配 tokio 运行时。
#[async_trait::async_trait]
pub trait CacheBackend: Send + Sync {
    /// 获取缓存原始字节
    async fn get_bytes(&self, key: &str) -> CacheResult<Option<Vec<u8>>>;

    /// 设置缓存原始字节
    async fn set_bytes(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> CacheResult<()>;

    /// 删除缓存键
    async fn delete(&self, key: &str) -> CacheResult<bool>;

    /// 检查键是否存在
    async fn exists(&self, key: &str) -> CacheResult<bool>;

    /// 设置键的过期时间
    async fn expire(&self, key: &str, ttl: Duration) -> CacheResult<bool>;

    /// 获取键的剩余过期时间
    async fn ttl(&self, key: &str) -> CacheResult<Option<Duration>>;

    /// 批量获取缓存原始字节
    async fn mget_bytes(&self, keys: &[&str]) -> CacheResult<Vec<Option<Vec<u8>>>>;

    /// 批量设置缓存原始字节
    async fn mset_bytes(&self, items: &[(&str, &[u8])], ttl: Option<Duration>) -> CacheResult<()>;

    /// 批量删除缓存键
    async fn mdelete(&self, keys: &[&str]) -> CacheResult<u64>;

    /// 自增操作
    async fn incr(&self, key: &str, delta: i64) -> CacheResult<i64>;

    /// 自减操作
    async fn decr(&self, key: &str, delta: i64) -> CacheResult<i64>;

    /// 清空当前命名空间下的所有缓存
    async fn clear(&self) -> CacheResult<()>;

    /// 获取缓存配置
    fn config(&self) -> &CacheConfig;
}

/// 缓存服务 (提供泛型封装)
pub struct CacheService {
    backend: Box<dyn CacheBackend>,
}

impl CacheService {
    /// 从后端创建缓存服务
    pub fn new(backend: Box<dyn CacheBackend>) -> Self {
        Self { backend }
    }

    /// 获取缓存值
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> CacheResult<Option<T>> {
        let bytes = self.backend.get_bytes(key).await?;
        match bytes {
            Some(data) => {
                let value =
                    serde_json::from_slice(&data).map_err(|_| WaeError::deserialization_failed(std::any::type_name::<T>()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// 设置缓存值
    pub async fn set<T: Serialize + ?Sized>(&self, key: &str, value: &T, ttl: Option<Duration>) -> CacheResult<()> {
        let data = serde_json::to_vec(value).map_err(|_| WaeError::serialization_failed(std::any::type_name::<T>()))?;
        self.backend.set_bytes(key, &data, ttl).await
    }

    /// 删除缓存键
    pub async fn delete(&self, key: &str) -> CacheResult<bool> {
        self.backend.delete(key).await
    }

    /// 检查键是否存在
    pub async fn exists(&self, key: &str) -> CacheResult<bool> {
        self.backend.exists(key).await
    }

    /// 设置键的过期时间
    pub async fn expire(&self, key: &str, ttl: Duration) -> CacheResult<bool> {
        self.backend.expire(key, ttl).await
    }

    /// 获取键的剩余过期时间
    pub async fn ttl(&self, key: &str) -> CacheResult<Option<Duration>> {
        self.backend.ttl(key).await
    }

    /// 批量获取缓存值
    pub async fn mget<T: DeserializeOwned>(&self, keys: &[&str]) -> CacheResult<Vec<Option<T>>> {
        let bytes_list = self.backend.mget_bytes(keys).await?;
        let mut results = Vec::with_capacity(bytes_list.len());
        for bytes in bytes_list {
            match bytes {
                Some(data) => {
                    let value = serde_json::from_slice(&data)
                        .map_err(|_| WaeError::deserialization_failed(std::any::type_name::<T>()))?;
                    results.push(Some(value));
                }
                None => results.push(None),
            }
        }
        Ok(results)
    }

    /// 批量设置缓存值
    pub async fn mset<T: Serialize + ?Sized>(&self, items: &[(&str, &T)], ttl: Option<Duration>) -> CacheResult<()> {
        let byte_items: Vec<(&str, Vec<u8>)> = items
            .iter()
            .map(|(k, v)| {
                let data = serde_json::to_vec(v).map_err(|_| WaeError::serialization_failed(std::any::type_name::<T>()))?;
                Ok((*k, data))
            })
            .collect::<CacheResult<_>>()?;

        let refs: Vec<(&str, &[u8])> = byte_items.iter().map(|(k, v)| (*k, v.as_slice())).collect();
        self.backend.mset_bytes(&refs, ttl).await
    }

    /// 批量删除缓存键
    pub async fn mdelete(&self, keys: &[&str]) -> CacheResult<u64> {
        self.backend.mdelete(keys).await
    }

    /// 自增操作
    pub async fn incr(&self, key: &str, delta: i64) -> CacheResult<i64> {
        self.backend.incr(key, delta).await
    }

    /// 自减操作
    pub async fn decr(&self, key: &str, delta: i64) -> CacheResult<i64> {
        self.backend.decr(key, delta).await
    }

    /// 清空缓存
    pub async fn clear(&self) -> CacheResult<()> {
        self.backend.clear().await
    }

    /// 获取配置
    pub fn config(&self) -> &CacheConfig {
        self.backend.config()
    }

    /// 构建带前缀的完整键
    pub fn build_key(&self, key: &str) -> String {
        let config = self.config();
        if config.key_prefix.is_empty() { key.to_string() } else { format!("{}:{}", config.key_prefix, key) }
    }
}

/// 内存缓存实现
pub mod memory {
    use super::*;
    use std::{collections::HashMap, sync::Arc};
    use tokio::{sync::RwLock, time::Instant};

    /// 缓存条目
    struct CacheEntry {
        data: Vec<u8>,
        expires_at: Option<Instant>,
    }

    impl CacheEntry {
        fn is_expired(&self) -> bool {
            self.expires_at.map(|exp| Instant::now() >= exp).unwrap_or(false)
        }
    }

    /// 内存缓存后端
    pub struct MemoryCacheBackend {
        config: CacheConfig,
        store: Arc<RwLock<HashMap<String, CacheEntry>>>,
    }

    impl MemoryCacheBackend {
        /// 创建新的内存缓存实例
        pub fn new(config: CacheConfig) -> Self {
            Self { config, store: Arc::new(RwLock::new(HashMap::new())) }
        }

        fn build_key(&self, key: &str) -> String {
            if self.config.key_prefix.is_empty() { key.to_string() } else { format!("{}:{}", self.config.key_prefix, key) }
        }
    }

    #[async_trait::async_trait]
    impl CacheBackend for MemoryCacheBackend {
        async fn get_bytes(&self, key: &str) -> CacheResult<Option<Vec<u8>>> {
            let full_key = self.build_key(key);
            let store = self.store.read().await;

            if let Some(entry) = store.get(&full_key) {
                if entry.is_expired() {
                    return Ok(None);
                }
                return Ok(Some(entry.data.clone()));
            }
            Ok(None)
        }

        async fn set_bytes(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> CacheResult<()> {
            let full_key = self.build_key(key);
            let effective_ttl = ttl.or(self.config.default_ttl);
            let expires_at = effective_ttl.map(|d| Instant::now() + d);

            let entry = CacheEntry { data: value.to_vec(), expires_at };
            let mut store = self.store.write().await;
            store.insert(full_key, entry);
            Ok(())
        }

        async fn delete(&self, key: &str) -> CacheResult<bool> {
            let full_key = self.build_key(key);
            let mut store = self.store.write().await;
            Ok(store.remove(&full_key).is_some())
        }

        async fn exists(&self, key: &str) -> CacheResult<bool> {
            let full_key = self.build_key(key);
            let store = self.store.read().await;
            if let Some(entry) = store.get(&full_key) {
                return Ok(!entry.is_expired());
            }
            Ok(false)
        }

        async fn expire(&self, key: &str, ttl: Duration) -> CacheResult<bool> {
            let full_key = self.build_key(key);
            let mut store = self.store.write().await;
            if let Some(entry) = store.get_mut(&full_key) {
                if entry.is_expired() {
                    return Ok(false);
                }
                entry.expires_at = Some(Instant::now() + ttl);
                return Ok(true);
            }
            Ok(false)
        }

        async fn ttl(&self, key: &str) -> CacheResult<Option<Duration>> {
            let full_key = self.build_key(key);
            let store = self.store.read().await;
            if let Some(entry) = store.get(&full_key) {
                if entry.is_expired() {
                    return Ok(None);
                }
                if let Some(expires_at) = entry.expires_at {
                    let now = Instant::now();
                    if expires_at > now {
                        return Ok(Some(expires_at - now));
                    }
                }
            }
            Ok(None)
        }

        async fn mget_bytes(&self, keys: &[&str]) -> CacheResult<Vec<Option<Vec<u8>>>> {
            let mut results = Vec::with_capacity(keys.len());
            for key in keys {
                results.push(self.get_bytes(key).await?);
            }
            Ok(results)
        }

        async fn mset_bytes(&self, items: &[(&str, &[u8])], ttl: Option<Duration>) -> CacheResult<()> {
            for (key, value) in items {
                self.set_bytes(key, value, ttl).await?;
            }
            Ok(())
        }

        async fn mdelete(&self, keys: &[&str]) -> CacheResult<u64> {
            let mut count = 0u64;
            for key in keys {
                if self.delete(key).await? {
                    count += 1;
                }
            }
            Ok(count)
        }

        async fn incr(&self, key: &str, delta: i64) -> CacheResult<i64> {
            let full_key = self.build_key(key);
            let mut store = self.store.write().await;

            let entry = store.entry(full_key.clone()).or_insert(CacheEntry { data: b"0".to_vec(), expires_at: None });

            let mut value: i64 = String::from_utf8_lossy(&entry.data).parse().unwrap_or(0);
            value += delta;
            entry.data = value.to_string().into_bytes();
            Ok(value)
        }

        async fn decr(&self, key: &str, delta: i64) -> CacheResult<i64> {
            self.incr(key, -delta).await
        }

        async fn clear(&self) -> CacheResult<()> {
            let mut store = self.store.write().await;
            if self.config.key_prefix.is_empty() {
                store.clear();
            }
            else {
                let prefix = format!("{}:", self.config.key_prefix);
                store.retain(|k, _| !k.starts_with(&prefix));
            }
            Ok(())
        }

        fn config(&self) -> &CacheConfig {
            &self.config
        }
    }
}

/// 便捷函数：创建内存缓存服务
pub fn memory_cache(config: CacheConfig) -> CacheService {
    CacheService::new(Box::new(memory::MemoryCacheBackend::new(config)))
}
