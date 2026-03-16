//! OpenAPI 相关功能

mod components;
mod paths;
mod security;

pub use components::*;
pub use paths::*;
pub use security::*;

/// OpenAPI 文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiInfo {
    /// API 标题
    pub title: String,
    /// API 版本
    pub version: String,
    /// API 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 服务条款 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    /// 联系信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// 许可信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

impl Default for OpenApiInfo {
    /// 创建默认的 OpenAPI 信息
    fn default() -> Self {
        Self {
            title: "API".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            terms_of_service: None,
            contact: None,
            license: None,
        }
    }
}

/// 联系信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// 联系人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 联系人邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 联系人 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 许可信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// 许可证名称
    pub name: String,
    /// 许可证 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 外部文档定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    /// 外部文档 URL
    pub url: String,
    /// 外部文档描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 标签定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签名称
    pub name: String,
    /// 标签描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 标签的外部文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

/// 服务器定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// 服务器 URL
    pub url: String,
    /// 服务器描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 服务器变量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::BTreeMap<String, ServerVariable>>,
}

/// 服务器变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    /// 默认值
    pub default: String,
    /// 枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    /// 描述
    #[serde(skip_serial