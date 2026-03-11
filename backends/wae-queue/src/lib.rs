//! WAE Queue - 消息队列抽象层
//!
//! 提供统一的消息队列能力抽象，支持多种消息队列后端。
//!
//! 深度融合 tokio 运行时，所有 API 都是异步优先设计。
//! 微服务架构友好，支持消息确认、重试、延迟队列等特性。

#![warn(missing_docs)]

use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;
use wae_types::{WaeError, WaeResult};

/// 消息 ID 类型
pub type MessageId = String;

/// 队列名称类型
pub type QueueName = String;

/// 消息元数据
#[derive(Debug, Clone, Default)]
pub struct MessageMetadata {
    /// 消息 ID
    pub id: Option<MessageId>,
    /// 关联 ID (用于消息关联)
    pub correlation_id: Option<String>,
    /// 回复队列
    pub reply_to: Option<QueueName>,
    /// 内容类型
    pub content_type: Option<String>,
    /// 时间戳
    pub timestamp: Option<u64>,
    /// 优先级 (0-9)
    pub priority: Option<u8>,
    /// 过期时间 (毫秒)
    pub expiration: Option<u64>,
    /// 自定义头信息
    pub headers: std::collections::HashMap<String, String>,
}

/// 原始消息 (字节形式)
#[derive(Debug, Clone)]
pub struct RawMessage {
    /// 消息数据
    pub data: Vec<u8>,
    /// 消息元数据
    pub metadata: MessageMetadata,
}

impl RawMessage {
    /// 创建新消息
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, metadata: MessageMetadata::default() }
    }

    /// 设置关联 ID
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.metadata.correlation_id = Some(id.into());
        self
    }

    /// 设置回复队列
    pub fn with_reply_to(mut self, queue: impl Into<String>) -> Self {
        self.metadata.reply_to = Some(queue.into());
        self
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.metadata.priority = Some(priority.min(9));
        self
    }

    /// 设置过期时间
    pub fn with_expiration(mut self, ms: u64) -> Self {
        self.metadata.expiration = Some(ms);
        self
    }

    /// 添加自定义头
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.headers.insert(key.into(), value.into());
        self
    }
}

/// 消息封装 (泛型)
#[derive(Debug, Clone)]
pub struct Message<T> {
    /// 消息体
    pub payload: T,
    /// 消息元数据
    pub metadata: MessageMetadata,
}

impl<T> Message<T> {
    /// 创建新消息
    pub fn new(payload: T) -> Self {
        Self { payload, metadata: MessageMetadata::default() }
    }

    /// 设置关联 ID
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.metadata.correlation_id = Some(id.into());
        self
    }

    /// 设置回复队列
    pub fn with_reply_to(mut self, queue: impl Into<String>) -> Self {
        self.metadata.reply_to = Some(queue.into());
        self
    }

    /// 设置优先级
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.metadata.priority = Some(priority.min(9));
        self
    }

    /// 设置过期时间
    pub fn with_expiration(mut self, ms: u64) -> Self {
        self.metadata.expiration = Some(ms);
        self
    }

    /// 添加自定义头
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.headers.insert(key.into(), value.into());
        self
    }

    /// 序列化为原始消息
    pub fn into_raw(self) -> WaeResult<RawMessage>
    where
        T: Serialize,
    {
        let data = serde_json::to_vec(&self.payload).map_err(|_e| WaeError::serialization_failed("Message"))?;
        Ok(RawMessage { data, metadata: self.metadata })
    }

    /// 序列化为原始消息 (引用版本)
    pub fn to_raw(&self) -> WaeResult<RawMessage>
    where
        T: Serialize,
    {
        let data = serde_json::to_vec(&self.payload).map_err(|_e| WaeError::serialization_failed("Message"))?;
        Ok(RawMessage { data, metadata: self.metadata.clone() })
    }
}

impl RawMessage {
    /// 反序列化为泛型消息
    pub fn into_typed<T: DeserializeOwned>(self) -> WaeResult<Message<T>> {
        let payload = serde_json::from_slice(&self.data).map_err(|_e| WaeError::deserialization_failed("Message"))?;
        Ok(Message { payload, metadata: self.metadata })
    }
}

/// 接收到的原始消息 (带确认能力)
#[derive(Debug)]
pub struct ReceivedRawMessage {
    /// 消息内容
    pub message: RawMessage,
    /// 消息 ID (用于确认)
    pub delivery_tag: u64,
    /// 重投递次数
    pub redelivery_count: u32,
}

/// 接收到的消息 (泛型)
#[derive(Debug)]
pub struct ReceivedMessage<T> {
    /// 消息内容
    pub message: Message<T>,
    /// 消息 ID (用于确认)
    pub delivery_tag: u64,
    /// 重投递次数
    pub redelivery_count: u32,
}

/// 队列配置
#[derive(Debug, Clone)]
pub struct QueueConfig {
    /// 队列名称
    pub name: QueueName,
    /// 是否持久化
    pub durable: bool,
    /// 是否自动删除 (当没有消费者时)
    pub auto_delete: bool,
    /// 最大消息数
    pub max_messages: Option<u64>,
    /// 最大消息大小 (字节)
    pub max_message_size: Option<u64>,
    /// 消息存活时间 (毫秒)
    pub message_ttl: Option<u64>,
    /// 死信队列
    pub dead_letter_queue: Option<QueueName>,
}

impl QueueConfig {
    /// 创建新的队列配置
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            durable: true,
            auto_delete: false,
            max_messages: None,
            max_message_size: None,
            message_ttl: None,
            dead_letter_queue: None,
        }
    }

    /// 设置持久化
    pub fn durable(mut self, durable: bool) -> Self {
        self.durable = durable;
        self
    }

    /// 设置自动删除
    pub fn auto_delete(mut self, auto_delete: bool) -> Self {
        self.auto_delete = auto_delete;
        self
    }

    /// 设置最大消息数
    pub fn max_messages(mut self, max: u64) -> Self {
        self.max_messages = Some(max);
        self
    }

    /// 设置消息 TTL
    pub fn message_ttl(mut self, ttl_ms: u64) -> Self {
        self.message_ttl = Some(ttl_ms);
        self
    }

    /// 设置死信队列
    pub fn dead_letter_queue(mut self, queue: impl Into<String>) -> Self {
        self.dead_letter_queue = Some(queue.into());
        self
    }
}

/// 生产者配置
#[derive(Debug, Clone)]
pub struct ProducerConfig {
    /// 默认队列
    pub default_queue: Option<QueueName>,
    /// 消息确认超时
    pub confirm_timeout: Duration,
    /// 重试次数
    pub retry_count: u32,
    /// 重试间隔
    pub retry_interval: Duration,
}

impl Default for ProducerConfig {
    fn default() -> Self {
        Self {
            default_queue: None,
            confirm_timeout: Duration::from_secs(5),
            retry_count: 3,
            retry_interval: Duration::from_millis(100),
        }
    }
}

/// 消费者配置
#[derive(Debug, Clone)]
pub struct ConsumerConfig {
    /// 队列名称
    pub queue: QueueName,
    /// 消费者标签
    pub consumer_tag: Option<String>,
    /// 是否自动确认
    pub auto_ack: bool,
    /// 预取数量
    pub prefetch_count: u16,
    /// 是否独占
    pub exclusive: bool,
}

impl ConsumerConfig {
    /// 创建新的消费者配置
    pub fn new(queue: impl Into<String>) -> Self {
        Self { queue: queue.into(), consumer_tag: None, auto_ack: false, prefetch_count: 10, exclusive: false }
    }

    /// 设置自动确认
    pub fn auto_ack(mut self, auto_ack: bool) -> Self {
        self.auto_ack = auto_ack;
        self
    }

    /// 设置预取数量
    pub fn prefetch(mut self, count: u16) -> Self {
        self.prefetch_count = count;
        self
    }
}

/// 消息生产者后端 trait (dyn 兼容)
#[async_trait::async_trait]
pub trait ProducerBackend: Send + Sync {
    /// 发送原始消息到指定队列
    async fn send_raw(&self, queue: &str, message: &RawMessage) -> WaeResult<MessageId>;

    /// 发送原始消息到默认队列
    async fn send_raw_default(&self, message: &RawMessage) -> WaeResult<MessageId>;

    /// 发送延迟消息
    async fn send_raw_delayed(&self, queue: &str, message: &RawMessage, delay: Duration) -> WaeResult<MessageId>;

    /// 批量发送消息
    async fn send_raw_batch(&self, queue: &str, messages: &[RawMessage]) -> WaeResult<Vec<MessageId>>;

    /// 获取生产者配置
    fn config(&self) -> &ProducerConfig;
}

/// 消息生产者 (提供泛型封装)
pub struct MessageProducer {
    backend: Box<dyn ProducerBackend>,
}

impl MessageProducer {
    /// 从后端创建生产者
    pub fn new(backend: Box<dyn ProducerBackend>) -> Self {
        Self { backend }
    }

    /// 发送消息到指定队列
    pub async fn send<T: Serialize + Send + Sync>(&self, queue: &str, message: &Message<T>) -> WaeResult<MessageId> {
        let raw = message.to_raw()?;
        self.backend.send_raw(queue, &raw).await
    }

    /// 发送消息到默认队列
    pub async fn send_default<T: Serialize + Send + Sync>(&self, message: &Message<T>) -> WaeResult<MessageId> {
        let raw = message.to_raw()?;
        self.backend.send_raw_default(&raw).await
    }

    /// 发送延迟消息
    pub async fn send_delayed<T: Serialize + Send + Sync>(
        &self,
        queue: &str,
        message: &Message<T>,
        delay: Duration,
    ) -> WaeResult<MessageId> {
        let raw = message.to_raw()?;
        self.backend.send_raw_delayed(queue, &raw, delay).await
    }

    /// 批量发送消息
    pub async fn send_batch<T: Serialize + Send + Sync>(
        &self,
        queue: &str,
        messages: &[Message<T>],
    ) -> WaeResult<Vec<MessageId>> {
        let raw_messages: Vec<RawMessage> = messages.iter().map(|m| m.to_raw()).collect::<WaeResult<_>>()?;
        self.backend.send_raw_batch(queue, &raw_messages).await
    }

    /// 获取配置
    pub fn config(&self) -> &ProducerConfig {
        self.backend.config()
    }
}

/// 消息消费者后端 trait (dyn 兼容)
#[async_trait::async_trait]
pub trait ConsumerBackend: Send + Sync {
    /// 接收原始消息
    async fn receive_raw(&self) -> WaeResult<Option<ReceivedRawMessage>>;

    /// 确认消息
    async fn ack(&self, delivery_tag: u64) -> WaeResult<()>;

    /// 拒绝消息
    async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()>;

    /// 获取消费者配置
    fn config(&self) -> &ConsumerConfig;
}

/// 消息消费者 (提供泛型封装)
pub struct MessageConsumer {
    backend: Box<dyn ConsumerBackend>,
}

impl MessageConsumer {
    /// 从后端创建消费者
    pub fn new(backend: Box<dyn ConsumerBackend>) -> Self {
        Self { backend }
    }

    /// 接收消息
    pub async fn receive<T: DeserializeOwned + Send>(&self) -> WaeResult<Option<ReceivedMessage<T>>> {
        let raw = match self.backend.receive_raw().await? {
            Some(r) => r,
            None => return Ok(None),
        };

        let message = raw.message.into_typed()?;
        Ok(Some(ReceivedMessage { message, delivery_tag: raw.delivery_tag, redelivery_count: raw.redelivery_count }))
    }

    /// 确认消息
    pub async fn ack(&self, delivery_tag: u64) -> WaeResult<()> {
        self.backend.ack(delivery_tag).await
    }

    /// 拒绝消息
    pub async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()> {
        self.backend.nack(delivery_tag, requeue).await
    }

    /// 获取配置
    pub fn config(&self) -> &ConsumerConfig {
        self.backend.config()
    }
}

/// 队列管理 trait
#[async_trait::async_trait]
pub trait QueueManager: Send + Sync {
    /// 声明队列
    async fn declare_queue(&self, config: &QueueConfig) -> WaeResult<()>;

    /// 删除队列
    async fn delete_queue(&self, name: &str) -> WaeResult<()>;

    /// 检查队列是否存在
    async fn queue_exists(&self, name: &str) -> WaeResult<bool>;

    /// 获取队列消息数量
    async fn queue_message_count(&self, name: &str) -> WaeResult<u64>;

    /// 清空队列
    async fn purge_queue(&self, name: &str) -> WaeResult<u64>;
}

/// 消息队列服务 trait

pub trait QueueService: Send + Sync {
    /// 创建生产者
    async fn create_producer(&self, config: ProducerConfig) -> WaeResult<MessageProducer>;

    /// 创建消费者
    async fn create_consumer(&self, config: ConsumerConfig) -> WaeResult<MessageConsumer>;

    /// 获取队列管理器
    fn manager(&self) -> &dyn QueueManager;

    /// 关闭连接
    async fn close(&self) -> WaeResult<()>;
}

/// 内存队列实现
pub mod memory {
    use super::*;
    use std::{
        collections::{HashMap, VecDeque},
        sync::Arc,
    };
    use tokio::sync::RwLock;

    /// 待处理消息状态
    struct PendingMessage {
        data: Vec<u8>,
        metadata: MessageMetadata,
        redelivery_count: u32,
        delivery_tag: u64,
    }

    /// 内存队列存储
    struct QueueStorage {
        messages: VecDeque<(Vec<u8>, MessageMetadata)>,
        pending_messages: HashMap<u64, PendingMessage>,
        next_delivery_tag: u64,
    }

    impl QueueStorage {
        fn new() -> Self {
            Self { messages: VecDeque::new(), pending_messages: HashMap::new(), next_delivery_tag: 1 }
        }
    }

    /// 内存队列管理器
    pub struct MemoryQueueManager {
        queues: Arc<RwLock<HashMap<String, QueueStorage>>>,
        configs: Arc<RwLock<HashMap<String, QueueConfig>>>,
    }

    impl MemoryQueueManager {
        /// 创建新的内存队列管理器
        pub fn new() -> Self {
            Self { queues: Arc::new(RwLock::new(HashMap::new())), configs: Arc::new(RwLock::new(HashMap::new())) }
        }
    }

    impl Default for MemoryQueueManager {
        fn default() -> Self {
            Self::new()
        }
    }

    #[async_trait::async_trait]
    impl QueueManager for MemoryQueueManager {
        async fn declare_queue(&self, config: &QueueConfig) -> WaeResult<()> {
            let mut queues = self.queues.write().await;
            let mut configs = self.configs.write().await;

            if !queues.contains_key(&config.name) {
                queues.insert(config.name.clone(), QueueStorage::new());
            }
            configs.insert(config.name.clone(), config.clone());
            Ok(())
        }

        async fn delete_queue(&self, name: &str) -> WaeResult<()> {
            let mut queues = self.queues.write().await;
            let mut configs = self.configs.write().await;
            queues.remove(name);
            configs.remove(name);
            Ok(())
        }

        async fn queue_exists(&self, name: &str) -> WaeResult<bool> {
            let queues = self.queues.read().await;
            Ok(queues.contains_key(name))
        }

        async fn queue_message_count(&self, name: &str) -> WaeResult<u64> {
            let queues = self.queues.read().await;
            Ok(queues.get(name).map(|q| q.messages.len() as u64 + q.pending_messages.len() as u64).unwrap_or(0))
        }

        async fn purge_queue(&self, name: &str) -> WaeResult<u64> {
            let mut queues = self.queues.write().await;
            if let Some(queue) = queues.get_mut(name) {
                let count = queue.messages.len() as u64 + queue.pending_messages.len() as u64;
                queue.messages.clear();
                queue.pending_messages.clear();
                return Ok(count);
            }
            Ok(0)
        }
    }

    /// 内存生产者后端
    pub struct MemoryProducerBackend {
        config: ProducerConfig,
        queues: Arc<RwLock<HashMap<String, QueueStorage>>>,
        manager: Arc<MemoryQueueManager>,
    }

    impl MemoryProducerBackend {
        /// 创建新的内存生产者后端
        pub fn new(config: ProducerConfig, manager: Arc<MemoryQueueManager>) -> Self {
            Self { config, queues: manager.queues.clone(), manager }
        }

        /// 内部发送消息到指定队列（不声明队列）
        async fn send_raw_internal(&self, queue: &str, data: Vec<u8>, mut metadata: MessageMetadata) -> WaeResult<MessageId> {
            let id = uuid::Uuid::new_v4().to_string();
            metadata.id = Some(id.clone());
            metadata.timestamp =
                Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64);

            let mut queues = self.queues.write().await;
            if let Some(q) = queues.get_mut(queue) {
                q.messages.push_back((data, metadata));
            }
            Ok(id)
        }
    }

    #[async_trait::async_trait]
    impl ProducerBackend for MemoryProducerBackend {
        async fn send_raw(&self, queue: &str, message: &RawMessage) -> WaeResult<MessageId> {
            self.manager.declare_queue(&QueueConfig::new(queue)).await?;

            let id = uuid::Uuid::new_v4().to_string();
            let mut metadata = message.metadata.clone();
            metadata.id = Some(id.clone());
            metadata.timestamp =
                Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64);

            let mut queues = self.queues.write().await;
            if let Some(q) = queues.get_mut(queue) {
                q.messages.push_back((message.data.clone(), metadata));
            }
            Ok(id)
        }

        async fn send_raw_default(&self, message: &RawMessage) -> WaeResult<MessageId> {
            let queue = self.config.default_queue.as_ref().ok_or_else(|| WaeError::config_missing("default_queue"))?;
            self.send_raw(queue, message).await
        }

        async fn send_raw_delayed(&self, queue: &str, message: &RawMessage, delay: Duration) -> WaeResult<MessageId> {
            tokio::time::sleep(delay).await;
            self.send_raw(queue, message).await
        }

        async fn send_raw_batch(&self, queue: &str, messages: &[RawMessage]) -> WaeResult<Vec<MessageId>> {
            let mut ids = Vec::with_capacity(messages.len());
            for msg in messages {
                ids.push(self.send_raw(queue, msg).await?);
            }
            Ok(ids)
        }

        fn config(&self) -> &ProducerConfig {
            &self.config
        }
    }

    /// 内存消费者后端
    pub struct MemoryConsumerBackend {
        config: ConsumerConfig,
        queues: Arc<RwLock<HashMap<String, QueueStorage>>>,
        manager: Arc<MemoryQueueManager>,
    }

    impl MemoryConsumerBackend {
        /// 创建新的内存消费者后端
        pub fn new(config: ConsumerConfig, manager: Arc<MemoryQueueManager>) -> Self {
            Self { config, queues: manager.queues.clone(), manager: manager.clone() }
        }
    }

    #[async_trait::async_trait]
    impl ConsumerBackend for MemoryConsumerBackend {
        async fn receive_raw(&self) -> WaeResult<Option<ReceivedRawMessage>> {
            let mut queues = self.queues.write().await;
            if let Some(queue) = queues.get_mut(&self.config.queue) {
                if let Some((data, metadata)) = queue.messages.pop_front() {
                    let delivery_tag = queue.next_delivery_tag;
                    queue.next_delivery_tag += 1;

                    let redelivery_count = metadata.headers.get("x-redelivery-count").and_then(|s| s.parse().ok()).unwrap_or(0);

                    let pending_msg =
                        PendingMessage { data: data.clone(), metadata: metadata.clone(), redelivery_count, delivery_tag };
                    queue.pending_messages.insert(delivery_tag, pending_msg);

                    let message = RawMessage { data, metadata };
                    return Ok(Some(ReceivedRawMessage { message, delivery_tag, redelivery_count }));
                }
            }
            Ok(None)
        }

        async fn ack(&self, delivery_tag: u64) -> WaeResult<()> {
            let mut queues = self.queues.write().await;
            if let Some(queue) = queues.get_mut(&self.config.queue) {
                queue.pending_messages.remove(&delivery_tag);
            }
            Ok(())
        }

        async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()> {
            let configs = self.manager.configs.read().await;
            let queue_config = configs.get(&self.config.queue).cloned();
            drop(configs);

            let mut queues = self.queues.write().await;

            if let Some(queue) = queues.get_mut(&self.config.queue) {
                if let Some(mut pending_msg) = queue.pending_messages.remove(&delivery_tag) {
                    if requeue {
                        pending_msg.redelivery_count += 1;
                        pending_msg
                            .metadata
                            .headers
                            .insert("x-redelivery-count".to_string(), pending_msg.redelivery_count.to_string());

                        queue.messages.push_back((pending_msg.data, pending_msg.metadata));
                    }
                    else {
                        if let Some(dlq_name) = queue_config.and_then(|c| c.dead_letter_queue) {
                            drop(queues);

                            self.manager.declare_queue(&QueueConfig::new(&dlq_name)).await?;

                            let mut queues = self.queues.write().await;
                            if let Some(dlq) = queues.get_mut(&dlq_name) {
                                dlq.messages.push_back((pending_msg.data, pending_msg.metadata));
                            }
                        }
                    }
                }
            }

            Ok(())
        }

        fn config(&self) -> &ConsumerConfig {
            &self.config
        }
    }

    /// 内存队列服务
    pub struct MemoryQueueService {
        manager: Arc<MemoryQueueManager>,
    }

    impl MemoryQueueService {
        /// 创建新的内存队列服务
        pub fn new() -> Self {
            Self { manager: Arc::new(MemoryQueueManager::new()) }
        }
    }

    impl Default for MemoryQueueService {
        fn default() -> Self {
            Self::new()
        }
    }

    impl QueueService for MemoryQueueService {
        async fn create_producer(&self, config: ProducerConfig) -> WaeResult<MessageProducer> {
            Ok(MessageProducer::new(Box::new(MemoryProducerBackend::new(config, self.manager.clone()))))
        }

        async fn create_consumer(&self, config: ConsumerConfig) -> WaeResult<MessageConsumer> {
            self.manager.declare_queue(&QueueConfig::new(&config.queue)).await?;
            Ok(MessageConsumer::new(Box::new(MemoryConsumerBackend::new(config, self.manager.clone()))))
        }

        fn manager(&self) -> &dyn QueueManager {
            self.manager.as_ref() as &dyn QueueManager
        }

        async fn close(&self) -> WaeResult<()> {
            Ok(())
        }
    }
}

/// Redis Streams 队列实现
#[cfg(feature = "redis-backend")]
pub mod redis_backend {
    use super::*;
    use ::redis::{AsyncCommands, Client, FromRedisValue, RedisResult, streams::StreamReadOptions};
    use base64::{Engine as _, engine::general_purpose};
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::Mutex;

    /// Redis 队列管理器
    pub struct RedisQueueManager {
        client: Client,
        configs: Arc<Mutex<HashMap<String, QueueConfig>>>,
    }

    impl RedisQueueManager {
        /// 创建新的 Redis 队列管理器
        pub fn new(client: Client) -> Self {
            Self { client, configs: Arc::new(Mutex::new(HashMap::new())) }
        }

        /// 从 URL 创建 Redis 队列管理器
        pub async fn from_url(url: &str) -> WaeResult<Self> {
            let client = Client::open(url).map_err(|e| WaeError::internal(format!("Failed to create Redis client: {}", e)))?;
            Ok(Self::new(client))
        }

        fn stream_name(queue: &str) -> String {
            format!("wae:stream:{}", queue)
        }

        fn group_name() -> &'static str {
            "wae-consumer-group"
        }
    }

    #[async_trait::async_trait]
    impl QueueManager for RedisQueueManager {
        async fn declare_queue(&self, config: &QueueConfig) -> WaeResult<()> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = Self::stream_name(&config.name);
            let group_name = Self::group_name();

            let _: RedisResult<()> = conn.xgroup_create_mkstream(&stream_name, group_name, "0").await;

            let mut configs = self.configs.lock().await;
            configs.insert(config.name.clone(), config.clone());

            Ok(())
        }

        async fn delete_queue(&self, name: &str) -> WaeResult<()> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = Self::stream_name(name);

            let _: RedisResult<()> = conn.del(&stream_name).await;

            let mut configs = self.configs.lock().await;
            configs.remove(name);

            Ok(())
        }

        async fn queue_exists(&self, name: &str) -> WaeResult<bool> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = Self::stream_name(name);
            let exists: bool = conn
                .exists(&stream_name)
                .await
                .map_err(|e| WaeError::internal(format!("Failed to check if stream exists: {}", e)))?;

            Ok(exists)
        }

        async fn queue_message_count(&self, name: &str) -> WaeResult<u64> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = Self::stream_name(name);
            let len: u64 =
                conn.xlen(&stream_name).await.map_err(|e| WaeError::internal(format!("Failed to get stream length: {}", e)))?;

            Ok(len)
        }

        async fn purge_queue(&self, name: &str) -> WaeResult<u64> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = Self::stream_name(name);
            let len: u64 =
                conn.xlen(&stream_name).await.map_err(|e| WaeError::internal(format!("Failed to get stream length: {}", e)))?;

            let _: RedisResult<()> = conn.del(&stream_name).await;

            Ok(len)
        }
    }

    /// Redis 生产者后端
    pub struct RedisProducerBackend {
        config: ProducerConfig,
        client: Client,
        manager: Arc<RedisQueueManager>,
    }

    impl RedisProducerBackend {
        /// 创建新的 Redis 生产者后端
        pub fn new(config: ProducerConfig, manager: Arc<RedisQueueManager>) -> Self {
            Self { config, client: manager.client.clone(), manager }
        }

        fn encode_metadata(metadata: &MessageMetadata) -> HashMap<String, String> {
            let mut fields = HashMap::new();

            if let Some(id) = &metadata.id {
                fields.insert("id".to_string(), id.clone());
            }
            if let Some(correlation_id) = &metadata.correlation_id {
                fields.insert("correlation_id".to_string(), correlation_id.clone());
            }
            if let Some(reply_to) = &metadata.reply_to {
                fields.insert("reply_to".to_string(), reply_to.clone());
            }
            if let Some(content_type) = &metadata.content_type {
                fields.insert("content_type".to_string(), content_type.clone());
            }
            if let Some(timestamp) = metadata.timestamp {
                fields.insert("timestamp".to_string(), timestamp.to_string());
            }
            if let Some(priority) = metadata.priority {
                fields.insert("priority".to_string(), priority.to_string());
            }
            if let Some(expiration) = metadata.expiration {
                fields.insert("expiration".to_string(), expiration.to_string());
            }

            for (key, value) in &metadata.headers {
                fields.insert(format!("header:{}", key), value.clone());
            }

            fields
        }
    }

    #[async_trait::async_trait]
    impl ProducerBackend for RedisProducerBackend {
        async fn send_raw(&self, queue: &str, message: &RawMessage) -> WaeResult<MessageId> {
            self.manager.declare_queue(&QueueConfig::new(queue)).await?;

            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = RedisQueueManager::stream_name(queue);
            let data_b64 = general_purpose::STANDARD.encode(&message.data);

            let mut fields_vec = Vec::new();
            let mut fields = Self::encode_metadata(&message.metadata);
            fields.insert("data".to_string(), data_b64);
            for (k, v) in fields {
                fields_vec.push((k, v));
            }

            let id: String = conn
                .xadd(&stream_name, "*", &fields_vec)
                .await
                .map_err(|e| WaeError::internal(format!("Failed to add message to stream: {}", e)))?;

            Ok(id)
        }

        async fn send_raw_default(&self, message: &RawMessage) -> WaeResult<MessageId> {
            let queue = self.config.default_queue.as_ref().ok_or_else(|| WaeError::config_missing("default_queue"))?;
            self.send_raw(queue, message).await
        }

        async fn send_raw_delayed(&self, queue: &str, message: &RawMessage, delay: Duration) -> WaeResult<MessageId> {
            let delayed_queue = format!("{}:delayed", queue);
            let delayed_stream = RedisQueueManager::stream_name(&delayed_queue);

            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let data_b64 = general_purpose::STANDARD.encode(&message.data);
            let mut fields = Self::encode_metadata(&message.metadata);
            fields.insert("data".to_string(), data_b64);
            fields.insert("target_queue".to_string(), queue.to_string());

            let mut fields_vec = Vec::new();
            for (k, v) in fields {
                fields_vec.push((k, v));
            }

            let id: String = conn
                .xadd(&delayed_stream, "*", &fields_vec)
                .await
                .map_err(|e| WaeError::internal(format!("Failed to add delayed message: {}", e)))?;

            let message_clone = message.clone();
            tokio::spawn({
                let client = self.client.clone();
                let manager = self.manager.clone();
                let queue = queue.to_string();
                let delayed_stream = delayed_stream.clone();
                let id = id.clone();
                let delay = delay;

                async move {
                    tokio::time::sleep(delay).await;

                    if let Ok(mut conn) = client.get_async_connection().await {
                        let _: RedisResult<()> = conn.xdel(&delayed_stream, &[&id]).await;

                        let mut producer = RedisProducerBackend::new(ProducerConfig::default(), manager.clone());
                        let _ = producer.send_raw(&queue, &message_clone).await;
                    }
                }
            });

            Ok(id)
        }

        async fn send_raw_batch(&self, queue: &str, messages: &[RawMessage]) -> WaeResult<Vec<MessageId>> {
            let mut ids = Vec::with_capacity(messages.len());
            for msg in messages {
                ids.push(self.send_raw(queue, msg).await?);
            }
            Ok(ids)
        }

        fn config(&self) -> &ProducerConfig {
            &self.config
        }
    }

    /// Redis 消费者后端
    pub struct RedisConsumerBackend {
        config: ConsumerConfig,
        client: Client,
        manager: Arc<RedisQueueManager>,
        consumer_name: String,
        delivery_tags: Arc<Mutex<HashMap<u64, String>>>,
        next_delivery_tag: Arc<Mutex<u64>>,
    }

    impl RedisConsumerBackend {
        /// 创建新的 Redis 消费者后端
        pub fn new(config: ConsumerConfig, manager: Arc<RedisQueueManager>) -> Self {
            let consumer_name = format!("wae-consumer-{}", uuid::Uuid::new_v4());
            Self {
                config,
                client: manager.client.clone(),
                manager,
                consumer_name,
                delivery_tags: Arc::new(Mutex::new(HashMap::new())),
                next_delivery_tag: Arc::new(Mutex::new(1)),
            }
        }

        fn decode_metadata(fields: &HashMap<String, String>) -> MessageMetadata {
            let mut metadata = MessageMetadata::default();

            if let Some(id) = fields.get("id") {
                metadata.id = Some(id.clone());
            }
            if let Some(correlation_id) = fields.get("correlation_id") {
                metadata.correlation_id = Some(correlation_id.clone());
            }
            if let Some(reply_to) = fields.get("reply_to") {
                metadata.reply_to = Some(reply_to.clone());
            }
            if let Some(content_type) = fields.get("content_type") {
                metadata.content_type = Some(content_type.clone());
            }
            if let Some(timestamp) = fields.get("timestamp").and_then(|s| s.parse().ok()) {
                metadata.timestamp = Some(timestamp);
            }
            if let Some(priority) = fields.get("priority").and_then(|s| s.parse().ok()) {
                metadata.priority = Some(priority);
            }
            if let Some(expiration) = fields.get("expiration").and_then(|s| s.parse().ok()) {
                metadata.expiration = Some(expiration);
            }

            for (key, value) in fields {
                if let Some(header_key) = key.strip_prefix("header:") {
                    metadata.headers.insert(header_key.to_string(), value.clone());
                }
            }

            metadata
        }
    }

    #[async_trait::async_trait]
    impl ConsumerBackend for RedisConsumerBackend {
        async fn receive_raw(&self) -> WaeResult<Option<ReceivedRawMessage>> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = RedisQueueManager::stream_name(&self.config.queue);
            let group_name = RedisQueueManager::group_name();

            let opts = StreamReadOptions::default().group(group_name, &self.consumer_name).count(1).block(1000);

            let result: RedisResult<redis::streams::StreamReadReply> = conn.xread_options(&[&stream_name], &[">"], &opts).await;

            match result {
                Ok(streams) => {
                    if let Some(stream) = streams.keys.into_iter().next() {
                        if let Some(entry) = stream.ids.into_iter().next() {
                            let mut fields = HashMap::new();
                            for (key, value) in entry.map {
                                if let Ok(s) = String::from_redis_value(&value) {
                                    fields.insert(key, s);
                                }
                            }

                            let data_b64 = fields.get("data").ok_or_else(|| WaeError::internal("Missing data field"))?;
                            let data = general_purpose::STANDARD
                                .decode(data_b64)
                                .map_err(|e| WaeError::internal(format!("Failed to decode data: {}", e)))?;

                            let metadata = Self::decode_metadata(&fields);

                            let mut next_tag = self.next_delivery_tag.lock().await;
                            let delivery_tag = *next_tag;
                            *next_tag += 1;
                            drop(next_tag);

                            let mut delivery_tags = self.delivery_tags.lock().await;
                            delivery_tags.insert(delivery_tag, entry.id.clone());

                            let redelivery_count =
                                metadata.headers.get("x-redelivery-count").and_then(|s| s.parse().ok()).unwrap_or(0);

                            let message = RawMessage { data, metadata };
                            return Ok(Some(ReceivedRawMessage { message, delivery_tag, redelivery_count }));
                        }
                    }
                    Ok(None)
                }
                Err(e) => {
                    if e.to_string().contains("NOGROUP") {
                        self.manager.declare_queue(&QueueConfig::new(&self.config.queue)).await?;
                        Ok(None)
                    }
                    else {
                        Err(WaeError::internal(format!("Failed to read from stream: {}", e)))
                    }
                }
            }
        }

        async fn ack(&self, delivery_tag: u64) -> WaeResult<()> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = RedisQueueManager::stream_name(&self.config.queue);
            let group_name = RedisQueueManager::group_name();

            let mut delivery_tags = self.delivery_tags.lock().await;
            if let Some(message_id) = delivery_tags.remove(&delivery_tag) {
                let _: RedisResult<()> = conn.xack(&stream_name, group_name, &[&message_id]).await;
                let _: RedisResult<()> = conn.xdel(&stream_name, &[&message_id]).await;
            }

            Ok(())
        }

        async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()> {
            let mut conn = self
                .client
                .get_async_connection()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to get Redis connection: {}", e)))?;

            let stream_name = RedisQueueManager::stream_name(&self.config.queue);
            let group_name = RedisQueueManager::group_name();

            let mut delivery_tags = self.delivery_tags.lock().await;
            if let Some(message_id) = delivery_tags.remove(&delivery_tag) {
                if requeue {
                    let _: RedisResult<()> =
                        conn.xclaim(&stream_name, group_name, &self.consumer_name, 0, &[&message_id]).await;
                }
                else {
                    let configs = self.manager.configs.lock().await;
                    let dlq_name = configs.get(&self.config.queue).and_then(|c| c.dead_letter_queue.clone());
                    drop(configs);

                    if let Some(dlq_name) = dlq_name {
                        let _: RedisResult<()> = conn.xack(&stream_name, group_name, &[&message_id]).await;
                        let _: RedisResult<()> = conn.xdel(&stream_name, &[&message_id]).await;
                    }
                    else {
                        let _: RedisResult<()> = conn.xack(&stream_name, group_name, &[&message_id]).await;
                        let _: RedisResult<()> = conn.xdel(&stream_name, &[&message_id]).await;
                    }
                }
            }

            Ok(())
        }

        fn config(&self) -> &ConsumerConfig {
            &self.config
        }
    }

    /// Redis 队列服务
    pub struct RedisQueueService {
        manager: Arc<RedisQueueManager>,
    }

    impl RedisQueueService {
        /// 创建新的 Redis 队列服务
        pub fn new(manager: Arc<RedisQueueManager>) -> Self {
            Self { manager }
        }

        /// 从 URL 创建 Redis 队列服务
        pub async fn from_url(url: &str) -> WaeResult<Self> {
            let manager = Arc::new(RedisQueueManager::from_url(url).await?);
            Ok(Self::new(manager))
        }
    }

    impl QueueService for RedisQueueService {
        async fn create_producer(&self, config: ProducerConfig) -> WaeResult<MessageProducer> {
            Ok(MessageProducer::new(Box::new(RedisProducerBackend::new(config, self.manager.clone()))))
        }

        async fn create_consumer(&self, config: ConsumerConfig) -> WaeResult<MessageConsumer> {
            self.manager.declare_queue(&QueueConfig::new(&config.queue)).await?;
            Ok(MessageConsumer::new(Box::new(RedisConsumerBackend::new(config, self.manager.clone()))))
        }

        fn manager(&self) -> &dyn QueueManager {
            self.manager.as_ref() as &dyn QueueManager
        }

        async fn close(&self) -> WaeResult<()> {
            Ok(())
        }
    }
}

/// 便捷函数：创建内存队列服务
pub fn memory_queue_service() -> memory::MemoryQueueService {
    memory::MemoryQueueService::new()
}

/// Kafka 队列实现
#[cfg(feature = "kafka-backend")]
pub mod kafka_backend {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};
    use rdkafka::{
        ClientConfig,
        consumer::{Consumer, DefaultConsumerContext, StreamConsumer},
        message::{Headers, OwnedMessage},
        producer::{DeliveryFuture, FutureProducer, FutureRecord},
        util::Timeout,
    };
    use std::{collections::HashMap, sync::Arc, time::Duration};
    use tokio::sync::Mutex;
    use uuid::Uuid;

    /// Kafka 连接配置
    #[derive(Debug, Clone)]
    pub struct KafkaConfig {
        /// Kafka Broker 地址 (例如: localhost:9092)
        pub brokers: String,
        /// 客户端 ID
        pub client_id: Option<String>,
        /// 生产者配置
        pub producer_config: HashMap<String, String>,
        /// 消费者配置
        pub consumer_config: HashMap<String, String>,
    }

    impl Default for KafkaConfig {
        fn default() -> Self {
            Self {
                brokers: "localhost:9092".to_string(),
                client_id: Some("wae-kafka".to_string()),
                producer_config: HashMap::new(),
                consumer_config: HashMap::new(),
            }
        }
    }

    impl KafkaConfig {
        /// 创建新的 Kafka 配置
        pub fn new(brokers: impl Into<String>) -> Self {
            Self { brokers: brokers.into(), ..Default::default() }
        }
    }

    /// Kafka 队列管理器
    pub struct KafkaQueueManager {
        config: ClientConfig,
    }

    impl KafkaQueueManager {
        /// 创建新的 Kafka 队列管理器
        pub fn new(config: &KafkaConfig) -> Self {
            let mut client_config = ClientConfig::new();
            client_config.set("bootstrap.servers", &config.brokers);
            if let Some(client_id) = &config.client_id {
                client_config.set("client.id", client_id);
            }
            for (key, value) in &config.producer_config {
                client_config.set(key, value);
            }
            for (key, value) in &config.consumer_config {
                client_config.set(key, value);
            }
            Self { config: client_config }
        }

        /// 内部主题名称
        fn topic_name(queue: &str) -> String {
            queue.to_string()
        }
    }

    #[async_trait::async_trait]
    impl QueueManager for KafkaQueueManager {
        async fn declare_queue(&self, config: &QueueConfig) -> WaeResult<()> {
            Ok(())
        }

        async fn delete_queue(&self, name: &str) -> WaeResult<()> {
            Ok(())
        }

        async fn queue_exists(&self, name: &str) -> WaeResult<bool> {
            Ok(true)
        }

        async fn queue_message_count(&self, name: &str) -> WaeResult<u64> {
            Ok(0)
        }

        async fn purge_queue(&self, name: &str) -> WaeResult<u64> {
            Ok(0)
        }
    }

    /// Kafka 生产者后端
    pub struct KafkaProducerBackend {
        config: ProducerConfig,
        producer: Arc<FutureProducer>,
        manager: Arc<KafkaQueueManager>,
    }

    impl KafkaProducerBackend {
        /// 创建新的 Kafka 生产者后端
        pub fn new(config: ProducerConfig, manager: Arc<KafkaQueueManager>, producer: Arc<FutureProducer>) -> Self {
            Self { config, producer, manager }
        }

        /// 编码元数据
        fn encode_metadata(metadata: &MessageMetadata) -> HashMap<String, String> {
            let mut fields = HashMap::new();

            if let Some(id) = &metadata.id {
                fields.insert("id".to_string(), id.clone());
            }
            if let Some(correlation_id) = &metadata.correlation_id {
                fields.insert("correlation_id".to_string(), correlation_id.clone());
            }
            if let Some(reply_to) = &metadata.reply_to {
                fields.insert("reply_to".to_string(), reply_to.clone());
            }
            if let Some(content_type) = &metadata.content_type {
                fields.insert("content_type".to_string(), content_type.clone());
            }
            if let Some(timestamp) = metadata.timestamp {
                fields.insert("timestamp".to_string(), timestamp.to_string());
            }
            if let Some(priority) = metadata.priority {
                fields.insert("priority".to_string(), priority.to_string());
            }
            if let Some(expiration) = metadata.expiration {
                fields.insert("expiration".to_string(), expiration.to_string());
            }

            for (key, value) in &metadata.headers {
                fields.insert(format!("header:{}", key), value.clone());
            }

            fields
        }
    }

    #[async_trait::async_trait]
    impl ProducerBackend for KafkaProducerBackend {
        async fn send_raw(&self, queue: &str, message: &RawMessage) -> WaeResult<MessageId> {
            let id = Uuid::new_v4().to_string();
            let mut metadata = message.metadata.clone();
            metadata.id = Some(id.clone());
            if metadata.timestamp.is_none() {
                metadata.timestamp =
                    Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64);
            }

            let topic = KafkaQueueManager::topic_name(queue);
            let data_b64 = general_purpose::STANDARD.encode(&message.data);
            let mut fields = Self::encode_metadata(&metadata);
            fields.insert("data".to_string(), data_b64);

            let payload = serde_json::to_vec(&fields)
                .map_err(|e| WaeError::internal(format!("Failed to serialize Kafka message: {}", e)))?;

            let record = FutureRecord::to(&topic).payload(&payload).key(&id);

            let result = self.producer.send(record, Timeout::After(Duration::from_secs(5))).await;

            match result {
                Ok((_, _)) => Ok(id),
                Err((e, _)) => Err(WaeError::internal(format!("Failed to send Kafka message: {}", e))),
            }
        }

        async fn send_raw_default(&self, message: &RawMessage) -> WaeResult<MessageId> {
            let queue = self.config.default_queue.as_ref().ok_or_else(|| WaeError::config_missing("default_queue"))?;
            self.send_raw(queue, message).await
        }

        async fn send_raw_delayed(&self, queue: &str, message: &RawMessage, delay: Duration) -> WaeResult<MessageId> {
            let id = Uuid::new_v4().to_string();
            let message_clone = message.clone();
            let producer = self.producer.clone();
            let manager = self.manager.clone();
            let queue = queue.to_string();

            tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                let mut producer_config = ProducerConfig::default();
                producer_config.default_queue = Some(queue.clone());
                let mut producer = KafkaProducerBackend::new(producer_config, manager.clone(), producer.clone());
                let _ = producer.send_raw(&queue, &message_clone).await;
            });

            Ok(id)
        }

        async fn send_raw_batch(&self, queue: &str, messages: &[RawMessage]) -> WaeResult<Vec<MessageId>> {
            let mut ids = Vec::with_capacity(messages.len());
            for msg in messages {
                ids.push(self.send_raw(queue, msg).await?);
            }
            Ok(ids)
        }

        fn config(&self) -> &ProducerConfig {
            &self.config
        }
    }

    /// Kafka 消费者后端
    pub struct KafkaConsumerBackend {
        config: ConsumerConfig,
        consumer: Arc<StreamConsumer<DefaultConsumerContext>>,
        manager: Arc<KafkaQueueManager>,
        delivery_tags: Arc<Mutex<HashMap<u64, (i32, i64)>>>,
        next_delivery_tag: Arc<Mutex<u64>>,
    }

    impl KafkaConsumerBackend {
        /// 创建新的 Kafka 消费者后端
        pub fn new(
            config: ConsumerConfig,
            manager: Arc<KafkaQueueManager>,
            consumer: Arc<StreamConsumer<DefaultConsumerContext>>,
        ) -> Self {
            Self {
                config,
                consumer,
                manager,
                delivery_tags: Arc::new(Mutex::new(HashMap::new())),
                next_delivery_tag: Arc::new(Mutex::new(1)),
            }
        }

        /// 解码元数据
        fn decode_metadata(fields: &HashMap<String, String>) -> MessageMetadata {
            let mut metadata = MessageMetadata::default();

            if let Some(id) = fields.get("id") {
                metadata.id = Some(id.clone());
            }
            if let Some(correlation_id) = fields.get("correlation_id") {
                metadata.correlation_id = Some(correlation_id.clone());
            }
            if let Some(reply_to) = fields.get("reply_to") {
                metadata.reply_to = Some(reply_to.clone());
            }
            if let Some(content_type) = fields.get("content_type") {
                metadata.content_type = Some(content_type.clone());
            }
            if let Some(timestamp) = fields.get("timestamp").and_then(|s| s.parse().ok()) {
                metadata.timestamp = Some(timestamp);
            }
            if let Some(priority) = fields.get("priority").and_then(|s| s.parse().ok()) {
                metadata.priority = Some(priority);
            }
            if let Some(expiration) = fields.get("expiration").and_then(|s| s.parse().ok()) {
                metadata.expiration = Some(expiration);
            }

            for (key, value) in fields {
                if let Some(header_key) = key.strip_prefix("header:") {
                    metadata.headers.insert(header_key.to_string(), value.clone());
                }
            }

            metadata
        }
    }

    #[async_trait::async_trait]
    impl ConsumerBackend for KafkaConsumerBackend {
        async fn receive_raw(&self) -> WaeResult<Option<ReceivedRawMessage>> {
            match tokio::time::timeout(Duration::from_secs(1), self.consumer.recv()).await {
                Ok(Ok(message)) => {
                    let payload = message.payload().ok_or_else(|| WaeError::internal("Kafka message missing payload"))?;
                    let fields: HashMap<String, String> = serde_json::from_slice(payload)
                        .map_err(|e| WaeError::internal(format!("Failed to deserialize Kafka message: {}", e)))?;
                    let data_b64 = fields.get("data").ok_or_else(|| WaeError::internal("Missing data field"))?;
                    let data = general_purpose::STANDARD
                        .decode(data_b64)
                        .map_err(|e| WaeError::internal(format!("Failed to decode data: {}", e)))?;
                    let metadata = Self::decode_metadata(&fields);

                    let mut next_tag = self.next_delivery_tag.lock().await;
                    let delivery_tag = *next_tag;
                    *next_tag += 1;
                    drop(next_tag);

                    let mut delivery_tags = self.delivery_tags.lock().await;
                    delivery_tags.insert(delivery_tag, (message.partition(), message.offset()));

                    let redelivery_count = metadata.headers.get("x-redelivery-count").and_then(|s| s.parse().ok()).unwrap_or(0);

                    let raw_message = RawMessage { data, metadata };
                    Ok(Some(ReceivedRawMessage { message: raw_message, delivery_tag, redelivery_count }))
                }
                Ok(Err(e)) => Err(WaeError::internal(format!("Failed to receive from Kafka: {}", e))),
                Err(_) => Ok(None),
            }
        }

        async fn ack(&self, delivery_tag: u64) -> WaeResult<()> {
            let mut delivery_tags = self.delivery_tags.lock().await;
            if let Some((partition, offset)) = delivery_tags.remove(&delivery_tag) {
                self.consumer
                    .commit_offset(
                        &rdkafka::TopicPartitionList::new().with_partition_offset(
                            &self.config.queue,
                            partition,
                            rdkafka::Offset::Offset(offset + 1),
                        ),
                        rdkafka::consumer::CommitMode::Async,
                    )
                    .map_err(|e| WaeError::internal(format!("Failed to commit Kafka offset: {}", e)))?;
            }
            Ok(())
        }

        async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()> {
            let mut delivery_tags = self.delivery_tags.lock().await;
            if let Some((partition, offset)) = delivery_tags.remove(&delivery_tag) {
                if requeue {
                    self.consumer
                        .seek(
                            &self.config.queue,
                            partition,
                            rdkafka::Offset::Offset(offset),
                            Timeout::After(Duration::from_secs(5)),
                        )
                        .await
                        .map_err(|e| WaeError::internal(format!("Failed to seek Kafka offset: {}", e)))?;
                }
                else {
                    self.consumer
                        .commit_offset(
                            &rdkafka::TopicPartitionList::new().with_partition_offset(
                                &self.config.queue,
                                partition,
                                rdkafka::Offset::Offset(offset + 1),
                            ),
                            rdkafka::consumer::CommitMode::Async,
                        )
                        .map_err(|e| WaeError::internal(format!("Failed to commit Kafka offset: {}", e)))?;
                }
            }
            Ok(())
        }

        fn config(&self) -> &ConsumerConfig {
            &self.config
        }
    }

    /// Kafka 队列服务
    pub struct KafkaQueueService {
        manager: Arc<KafkaQueueManager>,
        producer: Arc<FutureProducer>,
        consumer_config: ClientConfig,
    }

    impl KafkaQueueService {
        /// 创建新的 Kafka 队列服务
        pub async fn new(config: KafkaConfig) -> WaeResult<Self> {
            let manager = Arc::new(KafkaQueueManager::new(&config));

            let mut producer_config = ClientConfig::new();
            producer_config.set("bootstrap.servers", &config.brokers);
            if let Some(client_id) = &config.client_id {
                producer_config.set("client.id", client_id);
            }
            for (key, value) in &config.producer_config {
                producer_config.set(key, value);
            }
            let producer: FutureProducer =
                producer_config.create().map_err(|e| WaeError::internal(format!("Failed to create Kafka producer: {}", e)))?;

            let mut consumer_config = ClientConfig::new();
            consumer_config.set("bootstrap.servers", &config.brokers);
            consumer_config.set("group.id", format!("wae-{}", Uuid::new_v4()));
            consumer_config.set("enable.auto.commit", "false");
            consumer_config.set("auto.offset.reset", "earliest");
            if let Some(client_id) = &config.client_id {
                consumer_config.set("client.id", client_id);
            }
            for (key, value) in &config.consumer_config {
                consumer_config.set(key, value);
            }

            Ok(Self { manager, producer: Arc::new(producer), consumer_config })
        }
    }

    impl QueueService for KafkaQueueService {
        async fn create_producer(&self, config: ProducerConfig) -> WaeResult<MessageProducer> {
            Ok(MessageProducer::new(Box::new(KafkaProducerBackend::new(config, self.manager.clone(), self.producer.clone()))))
        }

        async fn create_consumer(&self, config: ConsumerConfig) -> WaeResult<MessageConsumer> {
            let consumer: StreamConsumer<DefaultConsumerContext> = self
                .consumer_config
                .create()
                .map_err(|e| WaeError::internal(format!("Failed to create Kafka consumer: {}", e)))?;

            consumer
                .subscribe(&[&config.queue])
                .map_err(|e| WaeError::internal(format!("Failed to subscribe to Kafka topic: {}", e)))?;

            let backend = KafkaConsumerBackend::new(config, self.manager.clone(), Arc::new(consumer));

            Ok(MessageConsumer::new(Box::new(backend)))
        }

        fn manager(&self) -> &dyn QueueManager {
            self.manager.as_ref() as &dyn QueueManager
        }

        async fn close(&self) -> WaeResult<()> {
            Ok(())
        }
    }
}

/// RabbitMQ (AMQP) 队列实现
#[cfg(feature = "rabbitmq-backend")]
pub mod rabbitmq {
    use super::*;
    use lapin::{
        BasicProperties, Channel, Connection, ConnectionProperties, message::Delivery, options::*,
        publisher_confirm::Confirmation, types::*,
    };
    use std::sync::Arc;
    use tokio::sync::Semaphore;
    use uuid::Uuid;

    /// RabbitMQ 连接配置
    #[derive(Clone)]
    pub struct RabbitMQConfig {
        /// AMQP URL (例如: amqp://guest:guest@localhost:5672/%2f)
        pub amqp_url: String,
        /// 连接属性
        pub connection_properties: ConnectionProperties,
    }

    impl Default for RabbitMQConfig {
        fn default() -> Self {
            Self {
                amqp_url: "amqp://guest:guest@localhost:5672/%2f".to_string(),
                connection_properties: ConnectionProperties::default(),
            }
        }
    }

    impl RabbitMQConfig {
        /// 创建新的 RabbitMQ 配置
        pub fn new(amqp_url: impl Into<String>) -> Self {
            Self { amqp_url: amqp_url.into(), connection_properties: ConnectionProperties::default() }
        }
    }

    /// RabbitMQ 队列管理器
    pub struct RabbitMQQueueManager {
        channel: Arc<Channel>,
    }

    impl RabbitMQQueueManager {
        /// 创建新的 RabbitMQ 队列管理器
        pub fn new(channel: Arc<Channel>) -> Self {
            Self { channel }
        }

        /// 内部声明队列（使用 lapin API）
        async fn declare_queue_internal(&self, config: &QueueConfig) -> WaeResult<()> {
            let mut arguments = FieldTable::default();

            if let Some(max_messages) = config.max_messages {
                arguments.insert("x-max-length".into(), AMQPValue::LongUInt(max_messages as u32));
            }

            if let Some(max_message_size) = config.max_message_size {
                arguments.insert("x-max-length-bytes".into(), AMQPValue::LongUInt(max_message_size as u32));
            }

            if let Some(message_ttl) = config.message_ttl {
                arguments.insert("x-message-ttl".into(), AMQPValue::LongUInt(message_ttl as u32));
            }

            if let Some(dlq) = &config.dead_letter_queue {
                arguments.insert("x-dead-letter-exchange".into(), AMQPValue::ShortString("".into()));
                arguments.insert("x-dead-letter-routing-key".into(), AMQPValue::ShortString(dlq.clone().into()));
            }

            self.channel
                .queue_declare(
                    &config.name,
                    QueueDeclareOptions {
                        passive: false,
                        durable: config.durable,
                        exclusive: false,
                        auto_delete: config.auto_delete,
                        nowait: false,
                    },
                    arguments,
                )
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ declare queue: {}", e)))?;

            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl QueueManager for RabbitMQQueueManager {
        async fn declare_queue(&self, config: &QueueConfig) -> WaeResult<()> {
            self.declare_queue_internal(config).await
        }

        async fn delete_queue(&self, name: &str) -> WaeResult<()> {
            self.channel
                .queue_delete(name, QueueDeleteOptions::default())
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ delete queue: {}", e)))?;
            Ok(())
        }

        async fn queue_exists(&self, name: &str) -> WaeResult<bool> {
            let result = self
                .channel
                .queue_declare(name, QueueDeclareOptions { passive: true, ..Default::default() }, FieldTable::default())
                .await;

            match result {
                Ok(_) => Ok(true),
                Err(lapin::Error::ProtocolError(_)) => Ok(false),
                Err(e) => Err(WaeError::internal(format!("RabbitMQ check queue exists: {}", e))),
            }
        }

        async fn queue_message_count(&self, name: &str) -> WaeResult<u64> {
            let queue = self
                .channel
                .queue_declare(name, QueueDeclareOptions { passive: true, ..Default::default() }, FieldTable::default())
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ get queue message count: {}", e)))?;

            Ok(queue.message_count() as u64)
        }

        async fn purge_queue(&self, name: &str) -> WaeResult<u64> {
            let result = self
                .channel
                .queue_purge(name, QueuePurgeOptions::default())
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ purge queue: {}", e)))?;

            Ok(result as u64)
        }
    }

    /// RabbitMQ 生产者后端
    pub struct RabbitMQProducerBackend {
        config: ProducerConfig,
        channel: Arc<Channel>,
        manager: Arc<RabbitMQQueueManager>,
    }

    impl RabbitMQProducerBackend {
        /// 创建新的 RabbitMQ 生产者后端
        pub fn new(config: ProducerConfig, channel: Arc<Channel>, manager: Arc<RabbitMQQueueManager>) -> Self {
            Self { config, channel, manager }
        }

        /// 转换元数据为 RabbitMQ 头信息
        fn metadata_to_properties(metadata: &MessageMetadata) -> BasicProperties {
            let mut props = BasicProperties::default();

            if let Some(correlation_id) = &metadata.correlation_id {
                props = props.with_correlation_id(correlation_id.clone().into());
            }

            if let Some(reply_to) = &metadata.reply_to {
                props = props.with_reply_to(reply_to.clone().into());
            }

            if let Some(content_type) = &metadata.content_type {
                props = props.with_content_type(content_type.clone().into());
            }

            if let Some(timestamp) = metadata.timestamp {
                props = props.with_timestamp(timestamp);
            }

            if let Some(priority) = metadata.priority {
                props = props.with_priority(priority);
            }

            if let Some(expiration) = metadata.expiration {
                props = props.with_expiration(expiration.to_string().into());
            }

            let mut headers = FieldTable::default();
            for (key, value) in &metadata.headers {
                headers.insert(key.clone().into(), AMQPValue::LongString(value.clone().into()));
            }

            if metadata.headers.len() > 0 {
                props = props.with_headers(headers);
            }

            props
        }

        /// 内部发送原始消息
        async fn send_raw_internal(&self, queue: &str, message: &RawMessage, id: &str) -> WaeResult<()> {
            let mut metadata = message.metadata.clone();
            if metadata.timestamp.is_none() {
                metadata.timestamp =
                    Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64);
            }

            let mut props = Self::metadata_to_properties(&metadata);
            props = props.with_message_id(id.into());

            let confirm = self
                .channel
                .basic_publish("", queue, BasicPublishOptions::default(), &message.data, props)
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ publish message: {}", e)))?
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ wait for confirm: {}", e)))?;

            match confirm {
                Confirmation::Ack(_) => Ok(()),
                Confirmation::Nack(_) => Err(WaeError::internal("RabbitMQ message nacked: Message was nacked by broker")),
                Confirmation::NotRequested => Ok(()),
            }
        }
    }

    #[async_trait::async_trait]
    impl ProducerBackend for RabbitMQProducerBackend {
        async fn send_raw(&self, queue: &str, message: &RawMessage) -> WaeResult<MessageId> {
            self.manager.declare_queue(&QueueConfig::new(queue)).await?;

            let id = Uuid::new_v4().to_string();
            self.send_raw_internal(queue, message, &id).await?;
            Ok(id)
        }

        async fn send_raw_default(&self, message: &RawMessage) -> WaeResult<MessageId> {
            let queue = self.config.default_queue.as_ref().ok_or_else(|| WaeError::config_missing("default_queue"))?;
            self.send_raw(queue, message).await
        }

        async fn send_raw_delayed(&self, queue: &str, message: &RawMessage, delay: Duration) -> WaeResult<MessageId> {
            let delay_ms = delay.as_millis() as u64;
            let delay_queue_name = format!("{}.delay.{}", queue, delay_ms);

            let mut delay_queue_config = QueueConfig::new(&delay_queue_name);
            delay_queue_config = delay_queue_config.dead_letter_queue(queue);
            delay_queue_config.message_ttl = Some(delay_ms);

            self.manager.declare_queue(&delay_queue_config).await?;

            let id = Uuid::new_v4().to_string();
            self.send_raw_internal(&delay_queue_name, message, &id).await?;
            Ok(id)
        }

        async fn send_raw_batch(&self, queue: &str, messages: &[RawMessage]) -> WaeResult<Vec<MessageId>> {
            let mut ids = Vec::with_capacity(messages.len());
            for msg in messages {
                ids.push(self.send_raw(queue, msg).await?);
            }
            Ok(ids)
        }

        fn config(&self) -> &ProducerConfig {
            &self.config
        }
    }

    /// RabbitMQ 消费者后端
    pub struct RabbitMQConsumerBackend {
        config: ConsumerConfig,
        channel: Arc<Channel>,
        manager: Arc<RabbitMQQueueManager>,
        consumer: Arc<tokio::sync::Mutex<lapin::Consumer>>,
        semaphore: Arc<Semaphore>,
    }

    impl RabbitMQConsumerBackend {
        /// 创建新的 RabbitMQ 消费者后端
        pub async fn new(config: ConsumerConfig, channel: Arc<Channel>, manager: Arc<RabbitMQQueueManager>) -> WaeResult<Self> {
            channel
                .basic_qos(config.prefetch_count, BasicQosOptions::default())
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ set qos: {}", e)))?;

            let consumer = channel
                .basic_consume(
                    &config.queue,
                    config.consumer_tag.as_deref().unwrap_or(""),
                    BasicConsumeOptions {
                        no_local: false,
                        no_ack: config.auto_ack,
                        exclusive: config.exclusive,
                        nowait: false,
                    },
                    FieldTable::default(),
                )
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ create consumer: {}", e)))?;

            let prefetch_count = config.prefetch_count;
            Ok(Self {
                config,
                channel,
                manager,
                consumer: Arc::new(tokio::sync::Mutex::new(consumer)),
                semaphore: Arc::new(Semaphore::new(prefetch_count as usize)),
            })
        }

        /// 转换 RabbitMQ 消息为原始消息
        fn delivery_to_raw_message(delivery: &Delivery) -> RawMessage {
            let mut metadata = MessageMetadata::default();

            metadata.id = delivery.properties.message_id().clone().map(|s| s.to_string());
            metadata.correlation_id = delivery.properties.correlation_id().clone().map(|s| s.to_string());
            metadata.reply_to = delivery.properties.reply_to().clone().map(|s| s.to_string());
            metadata.content_type = delivery.properties.content_type().clone().map(|s| s.to_string());
            metadata.timestamp = delivery.properties.timestamp().map(|t| t);
            metadata.priority = *delivery.properties.priority();
            metadata.expiration = delivery.properties.expiration().clone().and_then(|s| s.as_str().parse().ok());

            if let Some(headers) = delivery.properties.headers() {
                for (key, value) in headers.inner().iter() {
                    if let AMQPValue::LongString(s) = value {
                        metadata.headers.insert(key.to_string(), s.to_string());
                    }
                    else if let AMQPValue::ShortString(s) = value {
                        metadata.headers.insert(key.to_string(), s.to_string());
                    }
                }
            }

            RawMessage { data: delivery.data.clone(), metadata }
        }
    }

    #[async_trait::async_trait]
    impl ConsumerBackend for RabbitMQConsumerBackend {
        async fn receive_raw(&self) -> WaeResult<Option<ReceivedRawMessage>> {
            use futures::StreamExt;

            let mut consumer = self.consumer.lock().await;
            if let Some(delivery_result) = consumer.next().await {
                let delivery = delivery_result.map_err(|e| WaeError::internal(format!("RabbitMQ receive message: {}", e)))?;

                let message = Self::delivery_to_raw_message(&delivery);
                let redelivery_count = 0;

                Ok(Some(ReceivedRawMessage { message, delivery_tag: delivery.delivery_tag, redelivery_count }))
            }
            else {
                Ok(None)
            }
        }

        async fn ack(&self, delivery_tag: u64) -> WaeResult<()> {
            self.channel
                .basic_ack(delivery_tag, BasicAckOptions::default())
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ ack message: {}", e)))?;
            Ok(())
        }

        async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()> {
            self.channel
                .basic_nack(delivery_tag, BasicNackOptions { multiple: false, requeue })
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ nack message: {}", e)))?;
            Ok(())
        }

        fn config(&self) -> &ConsumerConfig {
            &self.config
        }
    }

    /// RabbitMQ 队列服务
    pub struct RabbitMQQueueService {
        connection: Arc<Connection>,
        channel: Arc<Channel>,
        manager: Arc<RabbitMQQueueManager>,
    }

    impl RabbitMQQueueService {
        /// 创建新的 RabbitMQ 队列服务
        pub async fn new(config: RabbitMQConfig) -> WaeResult<Self> {
            let connection = Connection::connect(&config.amqp_url, config.connection_properties)
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ connect: {}", e)))?;

            let channel =
                connection.create_channel().await.map_err(|e| WaeError::internal(format!("RabbitMQ create channel: {}", e)))?;

            channel
                .confirm_select(ConfirmSelectOptions::default())
                .await
                .map_err(|e| WaeError::internal(format!("RabbitMQ enable confirm mode: {}", e)))?;

            let connection = Arc::new(connection);
            let channel = Arc::new(channel);
            let manager = Arc::new(RabbitMQQueueManager::new(channel.clone()));

            Ok(Self { connection, channel, manager })
        }
    }

    impl QueueService for RabbitMQQueueService {
        async fn create_producer(&self, config: ProducerConfig) -> WaeResult<MessageProducer> {
            Ok(MessageProducer::new(Box::new(RabbitMQProducerBackend::new(config, self.channel.clone(), self.manager.clone()))))
        }

        async fn create_consumer(&self, config: ConsumerConfig) -> WaeResult<MessageConsumer> {
            self.manager.declare_queue(&QueueConfig::new(&config.queue)).await?;

            let backend = RabbitMQConsumerBackend::new(config, self.channel.clone(), self.manager.clone()).await?;

            Ok(MessageConsumer::new(Box::new(backend)))
        }

        fn manager(&self) -> &dyn QueueManager {
            self.manager.as_ref() as &dyn QueueManager
        }

        async fn close(&self) -> WaeResult<()> {
            self.connection.close(0, "").await.map_err(|e| WaeError::internal(format!("RabbitMQ close connection: {}", e)))?;
            Ok(())
        }
    }
}

/// Pulsar 队列实现
#[cfg(feature = "pulsar-backend")]
pub mod pulsar_backend {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};
    use pulsar::{Authentication, Pulsar, TokioExecutor, consumer, message::proto::MessageIdData, producer};
    use std::{collections::HashMap, sync::Arc, time::Duration};
    use tokio::sync::Mutex;
    use uuid::Uuid;

    /// Pulsar 连接配置
    #[derive(Debug, Clone)]
    pub struct PulsarConfig {
        /// Pulsar Broker 地址 (例如: pulsar://localhost:6650)
        pub brokers: String,
        /// 认证配置
        pub authentication: Option<Authentication>,
        /// 生产者配置
        pub producer_options: producer::ProducerOptions,
        /// 消费者配置
        pub consumer_options: consumer::ConsumerOptions,
    }

    impl Default for PulsarConfig {
        fn default() -> Self {
            Self {
                brokers: "pulsar://localhost:6650".to_string(),
                authentication: None,
                producer_options: producer::ProducerOptions::default(),
                consumer_options: consumer::ConsumerOptions::default(),
            }
        }
    }

    impl PulsarConfig {
        /// 创建新的 Pulsar 配置
        pub fn new(brokers: impl Into<String>) -> Self {
            Self { brokers: brokers.into(), ..Default::default() }
        }
    }

    /// Pulsar 队列管理器
    pub struct PulsarQueueManager {
        client: Arc<Pulsar<TokioExecutor>>,
    }

    impl PulsarQueueManager {
        /// 创建新的 Pulsar 队列管理器
        pub fn new(client: Arc<Pulsar<TokioExecutor>>) -> Self {
            Self { client }
        }

        /// 从配置创建 Pulsar 队列管理器
        pub async fn from_config(config: &PulsarConfig) -> WaeResult<Self> {
            let mut builder = Pulsar::builder(&config.brokers, TokioExecutor);
            if let Some(auth) = &config.authentication {
                builder = builder.with_auth(auth.clone());
            }
            let client =
                builder.build().await.map_err(|e| WaeError::internal(format!("Failed to create Pulsar client: {}", e)))?;
            Ok(Self::new(Arc::new(client)))
        }

        /// 内部主题名称
        fn topic_name(queue: &str) -> String {
            format!("persistent://public/default/{}", queue)
        }
    }

    #[async_trait::async_trait]
    impl QueueManager for PulsarQueueManager {
        async fn declare_queue(&self, _config: &QueueConfig) -> WaeResult<()> {
            Ok(())
        }

        async fn delete_queue(&self, _name: &str) -> WaeResult<()> {
            Ok(())
        }

        async fn queue_exists(&self, _name: &str) -> WaeResult<bool> {
            Ok(true)
        }

        async fn queue_message_count(&self, _name: &str) -> WaeResult<u64> {
            Ok(0)
        }

        async fn purge_queue(&self, _name: &str) -> WaeResult<u64> {
            Ok(0)
        }
    }

    /// Pulsar 生产者后端
    pub struct PulsarProducerBackend {
        config: ProducerConfig,
        producer: Arc<producer::Producer<TokioExecutor>>,
        manager: Arc<PulsarQueueManager>,
    }

    impl PulsarProducerBackend {
        /// 创建新的 Pulsar 生产者后端
        pub fn new(
            config: ProducerConfig,
            manager: Arc<PulsarQueueManager>,
            producer: Arc<producer::Producer<TokioExecutor>>,
        ) -> Self {
            Self { config, producer, manager }
        }

        /// 编码元数据
        fn encode_metadata(metadata: &MessageMetadata) -> HashMap<String, String> {
            let mut fields = HashMap::new();

            if let Some(id) = &metadata.id {
                fields.insert("id".to_string(), id.clone());
            }
            if let Some(correlation_id) = &metadata.correlation_id {
                fields.insert("correlation_id".to_string(), correlation_id.clone());
            }
            if let Some(reply_to) = &metadata.reply_to {
                fields.insert("reply_to".to_string(), reply_to.clone());
            }
            if let Some(content_type) = &metadata.content_type {
                fields.insert("content_type".to_string(), content_type.clone());
            }
            if let Some(timestamp) = metadata.timestamp {
                fields.insert("timestamp".to_string(), timestamp.to_string());
            }
            if let Some(priority) = metadata.priority {
                fields.insert("priority".to_string(), priority.to_string());
            }
            if let Some(expiration) = metadata.expiration {
                fields.insert("expiration".to_string(), expiration.to_string());
            }

            for (key, value) in &metadata.headers {
                fields.insert(format!("header:{}", key), value.clone());
            }

            fields
        }
    }

    #[async_trait::async_trait]
    impl ProducerBackend for PulsarProducerBackend {
        async fn send_raw(&self, queue: &str, message: &RawMessage) -> WaeResult<MessageId> {
            let id = Uuid::new_v4().to_string();
            let mut metadata = message.metadata.clone();
            metadata.id = Some(id.clone());
            if metadata.timestamp.is_none() {
                metadata.timestamp =
                    Some(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64);
            }

            let data_b64 = general_purpose::STANDARD.encode(&message.data);
            let mut fields = Self::encode_metadata(&metadata);
            fields.insert("data".to_string(), data_b64);

            let payload = serde_json::to_vec(&fields)
                .map_err(|e| WaeError::internal(format!("Failed to serialize Pulsar message: {}", e)))?;

            let message_id = self
                .producer
                .send(payload)
                .await
                .map_err(|e| WaeError::internal(format!("Failed to send Pulsar message: {}", e)))?
                .await
                .map_err(|e| WaeError::internal(format!("Failed to confirm Pulsar message: {}", e)))?;

            Ok(id)
        }

        async fn send_raw_default(&self, message: &RawMessage) -> WaeResult<MessageId> {
            let queue = self.config.default_queue.as_ref().ok_or_else(|| WaeError::config_missing("default_queue"))?;
            self.send_raw(queue, message).await
        }

        async fn send_raw_delayed(&self, queue: &str, message: &RawMessage, delay: Duration) -> WaeResult<MessageId> {
            let id = Uuid::new_v4().to_string();
            let message_clone = message.clone();
            let producer = self.producer.clone();
            let manager = self.manager.clone();
            let queue = queue.to_string();

            tokio::spawn(async move {
                tokio::time::sleep(delay).await;
                let mut producer_config = ProducerConfig::default();
                producer_config.default_queue = Some(queue.clone());
                let mut producer_backend = PulsarProducerBackend::new(producer_config, manager.clone(), producer.clone());
                let _ = producer_backend.send_raw(&queue, &message_clone).await;
            });

            Ok(id)
        }

        async fn send_raw_batch(&self, queue: &str, messages: &[RawMessage]) -> WaeResult<Vec<MessageId>> {
            let mut ids = Vec::with_capacity(messages.len());
            for msg in messages {
                ids.push(self.send_raw(queue, msg).await?);
            }
            Ok(ids)
        }

        fn config(&self) -> &ProducerConfig {
            &self.config
        }
    }

    /// Pulsar 消费者后端
    pub struct PulsarConsumerBackend {
        config: ConsumerConfig,
        consumer: Arc<Mutex<consumer::Consumer<TokioExecutor>>>,
        manager: Arc<PulsarQueueManager>,
        delivery_tags: Arc<Mutex<HashMap<u64, MessageIdData>>>,
        next_delivery_tag: Arc<Mutex<u64>>,
    }

    impl PulsarConsumerBackend {
        /// 创建新的 Pulsar 消费者后端
        pub fn new(
            config: ConsumerConfig,
            manager: Arc<PulsarQueueManager>,
            consumer: consumer::Consumer<TokioExecutor>,
        ) -> Self {
            Self {
                config,
                consumer: Arc::new(Mutex::new(consumer)),
                manager,
                delivery_tags: Arc::new(Mutex::new(HashMap::new())),
                next_delivery_tag: Arc::new(Mutex::new(1)),
            }
        }

        /// 解码元数据
        fn decode_metadata(fields: &HashMap<String, String>) -> MessageMetadata {
            let mut metadata = MessageMetadata::default();

            if let Some(id) = fields.get("id") {
                metadata.id = Some(id.clone());
            }
            if let Some(correlation_id) = fields.get("correlation_id") {
                metadata.correlation_id = Some(correlation_id.clone());
            }
            if let Some(reply_to) = fields.get("reply_to") {
                metadata.reply_to = Some(reply_to.clone());
            }
            if let Some(content_type) = fields.get("content_type") {
                metadata.content_type = Some(content_type.clone());
            }
            if let Some(timestamp) = fields.get("timestamp").and_then(|s| s.parse().ok()) {
                metadata.timestamp = Some(timestamp);
            }
            if let Some(priority) = fields.get("priority").and_then(|s| s.parse().ok()) {
                metadata.priority = Some(priority);
            }
            if let Some(expiration) = fields.get("expiration").and_then(|s| s.parse().ok()) {
                metadata.expiration = Some(expiration);
            }

            for (key, value) in fields {
                if let Some(header_key) = key.strip_prefix("header:") {
                    metadata.headers.insert(header_key.to_string(), value.clone());
                }
            }

            metadata
        }
    }

    #[async_trait::async_trait]
    impl ConsumerBackend for PulsarConsumerBackend {
        async fn receive_raw(&self) -> WaeResult<Option<ReceivedRawMessage>> {
            match tokio::time::timeout(Duration::from_secs(1), async {
                let mut consumer = self.consumer.lock().await;
                consumer.next().await
            })
            .await
            {
                Ok(Some(msg_result)) => {
                    let msg = msg_result.map_err(|e| WaeError::internal(format!("Failed to receive from Pulsar: {}", e)))?;
                    let payload = msg.payload.data.clone();
                    let fields: HashMap<String, String> = serde_json::from_slice(&payload)
                        .map_err(|e| WaeError::internal(format!("Failed to deserialize Pulsar message: {}", e)))?;
                    let data_b64 = fields.get("data").ok_or_else(|| WaeError::internal("Missing data field"))?;
                    let data = general_purpose::STANDARD
                        .decode(data_b64)
                        .map_err(|e| WaeError::internal(format!("Failed to decode data: {}", e)))?;
                    let metadata = Self::decode_metadata(&fields);

                    let mut next_tag = self.next_delivery_tag.lock().await;
                    let delivery_tag = *next_tag;
                    *next_tag += 1;
                    drop(next_tag);

                    let mut delivery_tags = self.delivery_tags.lock().await;
                    delivery_tags.insert(delivery_tag, msg.message_id.clone());

                    let redelivery_count = metadata.headers.get("x-redelivery-count").and_then(|s| s.parse().ok()).unwrap_or(0);

                    let raw_message = RawMessage { data, metadata };
                    Ok(Some(ReceivedRawMessage { message: raw_message, delivery_tag, redelivery_count }))
                }
                Ok(None) => Ok(None),
                Err(_) => Ok(None),
            }
        }

        async fn ack(&self, delivery_tag: u64) -> WaeResult<()> {
            let mut delivery_tags = self.delivery_tags.lock().await;
            if let Some(message_id) = delivery_tags.remove(&delivery_tag) {
                let mut consumer = self.consumer.lock().await;
                consumer
                    .ack(&message_id)
                    .await
                    .map_err(|e| WaeError::internal(format!("Failed to ack Pulsar message: {}", e)))?;
            }
            Ok(())
        }

        async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()> {
            let mut delivery_tags = self.delivery_tags.lock().await;
            if let Some(message_id) = delivery_tags.remove(&delivery_tag) {
                let mut consumer = self.consumer.lock().await;
                if requeue {
                    consumer
                        .nack_with_redelivery(&message_id)
                        .await
                        .map_err(|e| WaeError::internal(format!("Failed to nack Pulsar message with redelivery: {}", e)))?;
                }
                else {
                    consumer
                        .ack(&message_id)
                        .await
                        .map_err(|e| WaeError::internal(format!("Failed to ack Pulsar message in nack: {}", e)))?;
                }
            }
            Ok(())
        }

        fn config(&self) -> &ConsumerConfig {
            &self.config
        }
    }

    /// Pulsar 队列服务
    pub struct PulsarQueueService {
        manager: Arc<PulsarQueueManager>,
        client: Arc<Pulsar<TokioExecutor>>,
        producer_options: producer::ProducerOptions,
        consumer_options: consumer::ConsumerOptions,
    }

    impl PulsarQueueService {
        /// 创建新的 Pulsar 队列服务
        pub async fn new(config: PulsarConfig) -> WaeResult<Self> {
            let manager = PulsarQueueManager::from_config(&config).await?;
            let client = manager.client.clone();
            Ok(Self {
                manager: Arc::new(manager),
                client,
                producer_options: config.producer_options,
                consumer_options: config.consumer_options,
            })
        }
    }

    impl QueueService for PulsarQueueService {
        async fn create_producer(&self, config: ProducerConfig) -> WaeResult<MessageProducer> {
            let topic = config
                .default_queue
                .as_ref()
                .map(|q| PulsarQueueManager::topic_name(q))
                .unwrap_or_else(|| PulsarQueueManager::topic_name("default"));

            let producer: producer::Producer<TokioExecutor> = self
                .client
                .producer()
                .with_options(self.producer_options.clone())
                .with_topic(topic)
                .build()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to create Pulsar producer: {}", e)))?;

            Ok(MessageProducer::new(Box::new(PulsarProducerBackend::new(config, self.manager.clone(), Arc::new(producer)))))
        }

        async fn create_consumer(&self, config: ConsumerConfig) -> WaeResult<MessageConsumer> {
            let topic = PulsarQueueManager::topic_name(&config.queue);
            let consumer_name = config.consumer_tag.clone().unwrap_or_else(|| format!("wae-consumer-{}", Uuid::new_v4()));
            let subscription = format!("wae-subscription-{}", Uuid::new_v4());

            let consumer: consumer::Consumer<TokioExecutor> = self
                .client
                .consumer()
                .with_options(self.consumer_options.clone())
                .with_topic(topic)
                .with_consumer_name(consumer_name)
                .with_subscription(subscription)
                .with_subscription_type(consumer::SubscriptionType::Exclusive)
                .build()
                .await
                .map_err(|e| WaeError::internal(format!("Failed to create Pulsar consumer: {}", e)))?;

            let backend = PulsarConsumerBackend::new(config, self.manager.clone(), consumer);

            Ok(MessageConsumer::new(Box::new(backend)))
        }

        fn manager(&self) -> &dyn QueueManager {
            self.manager.as_ref() as &dyn QueueManager
        }

        async fn close(&self) -> WaeResult<()> {
            Ok(())
        }
    }
}
