//! WAE Monitoring - 告警服务抽象层
//!
//! 提供统一的告警能力抽象，支持多种告警后端。
//!
//! 深度融合 tokio 运行时，所有 API 都是异步优先设计。
//! 支持 AlertManager 和 PagerDuty 两种主流告警平台。

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wae_types::WaeError;

/// 告警操作结果类型
pub type AlertResult<T> = Result<T, WaeError>;

/// 告警级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// 信息级别
    Info,
    /// 警告级别
    Warning,
    /// 错误级别
    Error,
    /// 严重级别
    Critical,
}

impl Default for AlertSeverity {
    fn default() -> Self {
        Self::Warning
    }
}

/// 告警状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertStatus {
    /// 告警触发中
    Firing,
    /// 告警已解决
    Resolved,
}

/// 告警标签
pub type AlertLabels = HashMap<String, String>;

/// 告警注释
pub type AlertAnnotations = HashMap<String, String>;

/// 告警配置
#[derive(Debug, Clone)]
pub struct AlertConfig {
    /// 告警服务端点 URL
    pub endpoint: String,
    /// 超时时间
    pub timeout: std::time::Duration,
    /// 自定义 HTTP 头
    pub headers: HashMap<String, String>,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self { endpoint: String::new(), timeout: std::time::Duration::from_secs(30), headers: HashMap::new() }
    }
}

/// 告警消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// 告警名称/标识
    pub name: String,
    /// 告警级别
    pub severity: AlertSeverity,
    /// 告警状态
    pub status: AlertStatus,
    /// 告警标签
    pub labels: AlertLabels,
    /// 告警注释
    pub annotations: AlertAnnotations,
    /// 告警开始时间戳
    pub starts_at: Option<i64>,
    /// 告警结束时间戳
    pub ends_at: Option<i64>,
    /// 告警生成器 URL
    pub generator_url: Option<String>,
}

impl Alert {
    /// 创建新的告警
    pub fn new(name: String, severity: AlertSeverity) -> Self {
        Self {
            name,
            severity,
            status: AlertStatus::Firing,
            labels: HashMap::new(),
            annotations: HashMap::new(),
            starts_at: Some(chrono::Utc::now().timestamp()),
            ends_at: None,
            generator_url: None,
        }
    }

    /// 添加标签
    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    /// 添加注释
    pub fn with_annotation(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.annotations.insert(key.into(), value.into());
        self
    }

    /// 设置告警状态
    pub fn with_status(mut self, status: AlertStatus) -> Self {
        self.status = status;
        self
    }

    /// 设置生成器 URL
    pub fn with_generator_url(mut self, url: impl Into<String>) -> Self {
        self.generator_url = Some(url.into());
        self
    }
}

/// 告警后端核心 trait
///
/// 定义统一的告警操作接口，所有告警后端都需要实现此 trait。
#[async_trait::async_trait]
pub trait AlertBackend: Send + Sync {
    /// 发送单个告警
    async fn send_alert(&self, alert: Alert) -> AlertResult<()>;

    /// 批量发送告警
    async fn send_alerts(&self, alerts: &[Alert]) -> AlertResult<()>;

    /// 解决告警
    async fn resolve_alert(&self, alert_name: &str) -> AlertResult<()>;

    /// 获取配置
    fn config(&self) -> &AlertConfig;
}

/// 告警服务
pub struct AlertService {
    backend: Box<dyn AlertBackend>,
}

impl AlertService {
    /// 从后端创建告警服务
    pub fn new(backend: Box<dyn AlertBackend>) -> Self {
        Self { backend }
    }

    /// 发送单个告警
    pub async fn send_alert(&self, alert: Alert) -> AlertResult<()> {
        self.backend.send_alert(alert).await
    }

    /// 批量发送告警
    pub async fn send_alerts(&self, alerts: &[Alert]) -> AlertResult<()> {
        self.backend.send_alerts(alerts).await
    }

    /// 解决告警
    pub async fn resolve_alert(&self, alert_name: &str) -> AlertResult<()> {
        self.backend.resolve_alert(alert_name).await
    }

    /// 获取配置
    pub fn config(&self) -> &AlertConfig {
        self.backend.config()
    }
}

/// AlertManager 后端实现
#[cfg(feature = "alertmanager")]
pub mod alertmanager {
    use super::*;
    use serde_json::json;
    use url::Url;
    use wae_request::{HttpClient, HttpClientConfig};

    /// AlertManager 配置
    #[derive(Debug, Clone)]
    pub struct AlertManagerConfig {
        /// 基础配置
        pub base: AlertConfig,
    }

    impl From<AlertConfig> for AlertManagerConfig {
        fn from(base: AlertConfig) -> Self {
            Self { base }
        }
    }

    /// AlertManager 告警后端
    pub struct AlertManagerBackend {
        config: AlertManagerConfig,
        client: HttpClient,
    }

    impl AlertManagerBackend {
        /// 创建新的 AlertManager 后端
        pub fn new(config: AlertManagerConfig) -> AlertResult<Self> {
            let http_config = HttpClientConfig {
                timeout: config.base.timeout,
                connect_timeout: std::time::Duration::from_secs(10),
                user_agent: "wae-monitoring/0.1.0".to_string(),
                max_retries: 0,
                retry_delay: std::time::Duration::from_millis(1000),
                default_headers: config.base.headers.clone(),
            };
            let client = HttpClient::new(http_config);

            Ok(Self { config, client })
        }

        fn build_alert_url(&self) -> AlertResult<Url> {
            let mut url = Url::parse(&self.config.base.endpoint)
                .map_err(|e| WaeError::invalid_argument(format!("Invalid endpoint URL: {}", e)))?;

            url.set_path("/api/v2/alerts");
            Ok(url)
        }

        fn convert_to_alertmanager_alert(&self, alert: &Alert) -> serde_json::Value {
            let mut am_alert = json!({
                "labels": alert.labels.clone(),
                "annotations": alert.annotations.clone(),
            });

            am_alert["labels"]["alertname"] = json!(alert.name);
            am_alert["labels"]["severity"] = json!(match alert.severity {
                AlertSeverity::Info => "info",
                AlertSeverity::Warning => "warning",
                AlertSeverity::Error => "error",
                AlertSeverity::Critical => "critical",
            });

            if let Some(starts_at) = alert.starts_at {
                if let Some(dt) = chrono::DateTime::from_timestamp(starts_at, 0) {
                    am_alert["startsAt"] = json!(dt.to_rfc3339());
                }
            }

            if let Some(ends_at) = alert.ends_at {
                if let Some(dt) = chrono::DateTime::from_timestamp(ends_at, 0) {
                    am_alert["endsAt"] = json!(dt.to_rfc3339());
                }
            }

            if let Some(generator_url) = &alert.generator_url {
                am_alert["generatorURL"] = json!(generator_url);
            }

            am_alert
        }
    }

    #[async_trait::async_trait]
    impl AlertBackend for AlertManagerBackend {
        async fn send_alert(&self, alert: Alert) -> AlertResult<()> {
            self.send_alerts(&[alert]).await
        }

        async fn send_alerts(&self, alerts: &[Alert]) -> AlertResult<()> {
            let url = self.build_alert_url()?;

            let am_alerts: Vec<serde_json::Value> =
                alerts.iter().map(|alert| self.convert_to_alertmanager_alert(alert)).collect();

            let response = self.client.post_json(url.as_str(), &am_alerts).await?;

            if !response.is_success() {
                let status = response.status;
                let text = response.text().unwrap_or_else(|_| "No response body".to_string());
                return Err(WaeError::internal_error(format!("AlertManager returned error status {}: {}", status, text)));
            }

            Ok(())
        }

        async fn resolve_alert(&self, alert_name: &str) -> AlertResult<()> {
            let mut alert = Alert::new(alert_name.to_string(), AlertSeverity::Warning);
            alert.status = AlertStatus::Resolved;
            alert.ends_at = Some(chrono::Utc::now().timestamp());
            self.send_alert(alert).await
        }

        fn config(&self) -> &AlertConfig {
            &self.config.base
        }
    }
}

/// PagerDuty 后端实现
#[cfg(feature = "pagerduty")]
pub mod pagerduty {
    use super::*;
    use serde_json::json;
    use url::Url;
    use wae_request::{HttpClient, HttpClientConfig};

    /// PagerDuty 配置
    #[derive(Debug, Clone)]
    pub struct PagerDutyConfig {
        /// 基础配置
        pub base: AlertConfig,
        /// 集成密钥
        pub integration_key: String,
        /// 服务 ID（可选）
        pub service_id: Option<String>,
    }

    impl PagerDutyConfig {
        /// 创建新的 PagerDuty 配置
        pub fn new(endpoint: String, integration_key: String) -> Self {
            let base = AlertConfig { endpoint, ..Default::default() };
            Self { base, integration_key, service_id: None }
        }
    }

    /// PagerDuty 告警后端
    pub struct PagerDutyBackend {
        config: PagerDutyConfig,
        client: HttpClient,
    }

    impl PagerDutyBackend {
        /// 创建新的 PagerDuty 后端
        pub fn new(config: PagerDutyConfig) -> AlertResult<Self> {
            let http_config = HttpClientConfig {
                timeout: config.base.timeout,
                connect_timeout: std::time::Duration::from_secs(10),
                user_agent: "wae-monitoring/0.1.0".to_string(),
                max_retries: 0,
                retry_delay: std::time::Duration::from_millis(1000),
                default_headers: config.base.headers.clone(),
            };
            let client = HttpClient::new(http_config);

            Ok(Self { config, client })
        }

        fn build_event_url(&self) -> AlertResult<Url> {
            let endpoint = if self.config.base.endpoint.is_empty() {
                "https://events.pagerduty.com".to_string()
            }
            else {
                self.config.base.endpoint.clone()
            };

            let mut url =
                Url::parse(&endpoint).map_err(|e| WaeError::invalid_argument(format!("Invalid endpoint URL: {}", e)))?;

            url.set_path("/v2/enqueue");
            Ok(url)
        }

        fn convert_to_pagerduty_event(&self, alert: &Alert) -> serde_json::Value {
            let event_action = match alert.status {
                AlertStatus::Firing => "trigger",
                AlertStatus::Resolved => "resolve",
            };

            let severity = match alert.severity {
                AlertSeverity::Info => "info",
                AlertSeverity::Warning => "warning",
                AlertSeverity::Error => "error",
                AlertSeverity::Critical => "critical",
            };

            let mut event = json!({
                "routing_key": self.config.integration_key,
                "event_action": event_action,
                "dedup_key": alert.name,
                "payload": {
                    "summary": alert.name.clone(),
                    "severity": severity,
                    "source": alert.generator_url.clone().unwrap_or_else(|| "wae-monitoring".to_string()),
                    "custom_details": alert.annotations.clone(),
                }
            });

            event["payload"]["class"] = json!(alert.name.clone());

            if !alert.labels.is_empty() {
                event["payload"]["group"] = json!(alert.labels.get("group").cloned().unwrap_or_else(|| "default".to_string()));
            }

            event
        }
    }

    #[async_trait::async_trait]
    impl AlertBackend for PagerDutyBackend {
        async fn send_alert(&self, alert: Alert) -> AlertResult<()> {
            let url = self.build_event_url()?;

            let pd_event = self.convert_to_pagerduty_event(&alert);

            let mut headers = std::collections::HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());

            let body = serde_json::to_vec(&pd_event).map_err(|e| WaeError::serialization_failed("JSON"))?;

            let response = self.client.post_with_headers(url.as_str(), body, headers).await?;

            if !response.is_success() {
                let status = response.status;
                let text = response.text().unwrap_or_else(|_| "No response body".to_string());
                return Err(WaeError::internal_error(format!("PagerDuty returned error status {}: {}", status, text)));
            }

            Ok(())
        }

        async fn send_alerts(&self, alerts: &[Alert]) -> AlertResult<()> {
            for alert in alerts {
                self.send_alert(alert.clone()).await?;
            }
            Ok(())
        }

        async fn resolve_alert(&self, alert_name: &str) -> AlertResult<()> {
            let mut alert = Alert::new(alert_name.to_string(), AlertSeverity::Warning);
            alert.status = AlertStatus::Resolved;
            self.send_alert(alert).await
        }

        fn config(&self) -> &AlertConfig {
            &self.config.base
        }
    }
}

/// 便捷函数：创建 AlertManager 告警服务
#[cfg(feature = "alertmanager")]
pub fn alertmanager_service(config: alertmanager::AlertManagerConfig) -> AlertResult<AlertService> {
    let backend = alertmanager::AlertManagerBackend::new(config)?;
    Ok(AlertService::new(Box::new(backend)))
}

/// 便捷函数：创建 PagerDuty 告警服务
#[cfg(feature = "pagerduty")]
pub fn pagerduty_service(config: pagerduty::PagerDutyConfig) -> AlertResult<AlertService> {
    let backend = pagerduty::PagerDutyBackend::new(config)?;
    Ok(AlertService::new(Box::new(backend)))
}
