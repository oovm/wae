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
        let data = serde_json::to_vec(&self.payload).map_err(|e| WaeError::serialization_failed("Message"))?;
        Ok(RawMessage { data, metadata: self.metadata })
    }

    /// 序列化为原始消息 (引用版本)
    pub fn to_raw(&self) -> WaeResult<RawMessage>
    where
        T: Serialize,
    {
        let data = serde_json::to_vec(&self.payload).map_err(|e| WaeError::serialization_failed("Message"))?;
        Ok(RawMessage { data, metadata: self.metadata.clone() })
    }
}

impl RawMessage {
    /// 反序列化为泛型消息
    pub fn into_typed<T: DeserializeOwned>(self) -> WaeResult<Message<T>> {
        let payload = serde_json::from_slice(&self.data).map_err(|e| WaeError::deserialization_failed("Message"))?;
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

    /// 内存队列存储
    struct QueueStorage {
        messages: VecDeque<(u64, Vec<u8>, MessageMetadata)>,
        next_delivery_tag: u64,
    }

    impl QueueStorage {
        fn new() -> Self {
            Self { messages: VecDeque::new(), next_delivery_tag: 1 }
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
            Ok(queues.get(name).map(|q| q.messages.len() as u64).unwrap_or(0))
        }

        async fn purge_queue(&self, name: &str) -> WaeResult<u64> {
            let mut queues = self.queues.write().await;
            if let Some(queue) = queues.get_mut(name) {
                let count = queue.messages.len() as u64;
                queue.messages.clear();
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
                q.messages.push_back((q.next_delivery_tag, message.data.clone(), metadata));
                q.next_delivery_tag += 1;
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
    }

    impl MemoryConsumerBackend {
        /// 创建新的内存消费者后端
        pub fn new(config: ConsumerConfig, manager: Arc<MemoryQueueManager>) -> Self {
            Self { config, queues: manager.queues.clone() }
        }
    }

    #[async_trait::async_trait]
    impl ConsumerBackend for MemoryConsumerBackend {
        async fn receive_raw(&self) -> WaeResult<Option<ReceivedRawMessage>> {
            let mut queues = self.queues.write().await;
            if let Some(queue) = queues.get_mut(&self.config.queue) {
                if let Some((delivery_tag, data, metadata)) = queue.messages.pop_front() {
                    let message = RawMessage { data, metadata };
                    return Ok(Some(ReceivedRawMessage { message, delivery_tag, redelivery_count: 0 }));
                }
            }
            Ok(None)
        }

        async fn ack(&self, _delivery_tag: u64) -> WaeResult<()> {
            Ok(())
        }

        async fn nack(&self, _delivery_tag: u64, _requeue: bool) -> WaeResult<()> {
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

/// 便捷函数：创建内存队列服务
pub fn memory_queue_service() -> memory::MemoryQueueService {
    memory::MemoryQueueService::new()
}
