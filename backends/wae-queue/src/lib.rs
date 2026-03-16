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

// 模块声明
mod types;
mod producers;
mod consumers;
mod services;

// 重新导出所有公共类型和结构体
pub use types::*;
pub use producers::{ProducerBackend, MessageProducer};
pub use consumers::{ConsumerBackend, MessageConsumer};
pub use services::{QueueManager, QueueService};

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
            Ok(None)
        }

        async fn ack(&self, delivery_tag: u64) -> WaeResult<()> {
            Ok(())
        }

        async fn nack(&self, delivery_tag: u64, requeue: bool) -> WaeResult<()> {
            Ok(())
        }

        fn config(&self) -> &ConsumerConfig {
            &self.config
        }
    }

    /// Kafka 队列服务
    pub struct KafkaQueueService {
        manager: Arc<KafkaQueueManager>,
    }

    impl KafkaQueueService {
        /// 创建新的 Kafka 队列服务
        pub fn new(manager: Arc<KafkaQueueManager>) -> Self {
            Self { manager }
        }

        /// 从配置创建 Kafka 队列服务
        pub fn from_config(config: &KafkaConfig) -> Self {
            let manager = Arc::new(KafkaQueueManager::new(config));
            Self::new(manager)
        }
    }

    impl QueueService for KafkaQueueService {
        async fn create_producer(&self, config: ProducerConfig) -> WaeResult<MessageProducer> {
            let producer: FutureProducer = self
                .manager
                .config
                .create()
                .map_err(|e| WaeError::internal(format!("Failed to create Kafka producer: {}", e)))?;
            Ok(MessageProducer::new(Box::new(KafkaProducerBackend::new(
                config,
                self.manager.clone(),
                Arc::new(producer),
            ))))
        }

        async fn create_consumer(&self, config: ConsumerConfig) -> WaeResult<MessageConsumer> {
            let mut consumer_config = self.manager.config.clone();
            consumer_config.set("group.id", format!("wae-consumer-group-{}", uuid::Uuid::new_v4()));
            consumer_config.set("auto.offset.reset", "earliest");

            let consumer: StreamConsumer = consumer_config
                .create()
                .map_err(|e| WaeError::internal(format!("Failed to create Kafka consumer: {}", e)))?;

            consumer
                .subscribe(&[&config.queue])
                .map_err(|e| WaeError::internal(format!("Failed to subscribe to topic: {}", e)))?;

            Ok(MessageConsumer::new(Box::new(KafkaConsumerBackend::new(
                config,
                self.manager.clone(),
                Arc::new(consumer),
            ))))
        }

        fn manager(&self) -> &dyn QueueManager {
            self.manager.as_ref() as &dyn QueueManager
        }

        async fn close(&self) -> WaeResult<()> {
            Ok(())
        }
    }
}
