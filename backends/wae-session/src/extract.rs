//! Session 提取器
//!
//! 提供 axum 的 Session 提取器实现。

use crate::Session;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

/// Session 提取器
///
/// 用于在 axum 处理函数中提取 Session。
///
/// # 示例
///
/// ```rust,ignore
/// use wae_session::SessionExtractor;
///
/// async fn handler(session: SessionExtractor) -> impl IntoResponse {
///     let user_id: Option<String> = session.get_typed("user_id").await;
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SessionExtractor {
    /// Session 引用
    session: Arc<Session>,
}

impl SessionExtractor {
    /// 获取 Session 引用
    pub fn inner(&self) -> &Session {
        &self.session
    }

    /// 获取 Session ID
    pub fn id(&self) -> &str {
        self.session.id()
    }

    /// 检查是否是新创建的 Session
    pub fn is_new(&self) -> bool {
        self.session.is_new()
    }

    /// 获取值
    pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.session.get(key).await
    }

    /// 获取类型化的值
    pub async fn get_typed<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.session.get_typed(key).await
    }

    /// 设置值
    pub async fn set<T: serde::Serialize>(&self, key: impl Into<String>, value: T) {
        self.session.set(key, value).await
    }

    /// 删除值
    pub async fn remove(&self, key: &str) -> Option<serde_json::Value> {
        self.session.remove(key).await
    }

    /// 检查键是否存在
    pub async fn contains(&self, key: &str) -> bool {
        self.session.contains(key).await
    }

    /// 清空所有数据
    pub async fn clear(&self) {
        self.session.clear().await
    }

    /// 获取所有键
    pub async fn keys(&self) -> Vec<String> {
        self.session.keys().await
    }

    /// 获取数据条目数量
    pub async fn len(&self) -> usize {
        self.session.len().await
    }

    /// 检查是否为空
    pub async fn is_empty(&self) -> bool {
        self.session.is_empty().await
    }
}

impl<S> FromRequestParts<S> for SessionExtractor
where
    S: Send + Sync,
{
    type Rejection = SessionRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Arc<Session>>()
            .cloned()
            .map(|session| SessionExtractor { session })
            .ok_or(SessionRejection::MissingSession)
    }
}

/// Session 提取器拒绝错误
#[derive(Debug, Clone)]
pub enum SessionRejection {
    /// Session 不存在
    MissingSession,
}

impl std::fmt::Display for SessionRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionRejection::MissingSession => write!(f, "Session not found in request extensions"),
        }
    }
}

impl std::error::Error for SessionRejection {}

impl axum::response::IntoResponse for SessionRejection {
    fn into_response(self) -> axum::response::Response {
        let body = axum::body::Body::from(self.to_string());
        axum::http::Response::builder().status(axum::http::StatusCode::INTERNAL_SERVER_ERROR).body(body).unwrap()
    }
}
