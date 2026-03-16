//! WAE Schema - Schema 定义与验证模块
//!
//! 提供统一的 Schema 定义能力，支持：
//! - 数据结构 Schema 定义
//! - Schema 验证
//! - OpenAPI 文档生成
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// OpenAPI 相关功能
pub mod openapi;
/// Swagger UI 相关功能
pub mod swagger_ui;
use serde_json;

#[cfg(feature = "yaml")]
use serde_yaml;

/// Schema 生成 trait
///
/// 实现此 trait 的类型可以自动生成 OpenAPI Schema。
pub trait ToSchema {
    /// 生成 Schema 定义
    fn schema() -> Schema;
}

/// Schema 类型定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    /// 字符串类型
    String,
    /// 整数类型
    Integer,
    /// 浮点数类型
    Number,
    /// 布尔类型
    Boolean,
    /// 数组类型
    Array,
    /// 对象类型
    Object,
    /// 空值类型
    Null,
}

impl Default for SchemaType {
    /// 创建默认的 Schema 类型（Object）
    fn default() -> Self {
        Self::Object
    }
}

/// Schema 定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    /// Schema 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Schema 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Schema 类型
    #[serde(rename = "type")]
    pub schema_type: SchemaType,
    /// 属性定义（仅 Object 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::BTreeMap<String, Schema>>,
    /// 必需属性列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    /// 数组元素类型（仅 Array 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    /// 枚举值
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<serde_json::Value>>,
    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    /// 示例值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// 是否可为空
    #[serde(default)]
    pub nullable: bool,
    /// 只读
    #[serde(default)]
    pub read_only: bool,
    /// 只写
    #[serde(default)]
    pub write_only: bool,
    /// 最小值（Number/Integer）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// 最大值（Number/Integer）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    /// 最小长度（String）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    /// 最大长度（String）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    /// 正则模式（String）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// 格式（String）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 多个类型（用于 oneOf）
    #[serde(rename = "oneOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<Schema>>,
    /// 多个类型（用于 anyOf）
    #[serde(rename = "anyOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<Schema>>,
    /// 多个类型（用于 allOf）
    #[serde(rename = "allOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<Schema>>,
}

impl Default for Schema {
    /// 创建默认的 Schema（Object 类型）
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            schema_type: SchemaType::Object,
            properties: None,
            required: None,
            items: None,
            enum_values: None,
            default: None,
            example: None,
            nullable: false,
            read_only: false,
            write_only: false,
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
            pattern: None,
            format: None,
            one_of: None,
            any_of: None,
            all_of: None,
        }
    }
}

impl Schema {
    /// 创建字符串 Schema
    pub fn string() -> Self {
        Self { schema_type: SchemaType::String, ..Default::default() }
    }

    /// 创建整数 Schema
    pub fn integer() -> Self {
        Self { schema_type: SchemaType::Integer, ..Default::default() }
    }

    /// 创建数字 Schema
    pub fn number() -> Self {
        Self { schema_type: SchemaType::Number, ..Default::default() }
    }

    /// 创建布尔 Schema
    pub fn boolean() -> Self {
        Self { schema_type: SchemaType::Boolean, ..Default::default() }
    }

    /// 创建数组 Schema
    pub fn array(items: Schema) -> Self {
        Self { schema_type: SchemaType::Array, items: Some(Box::new(items)), ..Default::default() }
    }

    /// 创建对象 Schema
    pub fn object() -> Self {
        Self { schema_type: SchemaType::Object, properties: Some(std::collections::BTreeMap::new()), ..Default::default() }
    }

    /// 设置标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// 添加属性
    pub fn property(mut self, name: impl Into<String>, schema: Schema) -> Self {
        let properties = self.properties.get_or_insert_with(std::collections::BTreeMap::new);
        properties.insert(name.into(), schema);
        self
    }

    /// 设置必需属性
    pub fn required(mut self, fields: Vec<&str>) -> Self {
        self.required = Some(fields.into_iter().map(String::from).collect());
        self
    }

    /// 设置枚举值
    pub fn enum_values(mut self, values: Vec<serde_json::Value>) -> Self {
        self.enum_values = Some(values);
        self
    }

    /// 设置默认值
    pub fn with_default(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self
    }

    /// 设置示例值
    pub fn example(mut self, value: serde_json::Value) -> Self {
        self.example = Some(value);
        self
    }

    /// 设置格式
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// 设置正则模式
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// 设置最小长度
    pub fn min_length(mut self, len: usize) -> Self {
        self.min_length = Some(len);
        self
    }

    /// 设置最大长度
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    /// 设置最小值
    pub fn minimum(mut self, min: f64) -> Self {
        self.minimum = Some(min);
        self
    }

    /// 设置最大值
    pub fn maximum(mut self, max: f64) -> Self {
        self.maximum = Some(max);
        self
    }

    /// 设置可为空
    pub fn nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }

    /// 设置只读
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// 设置只写
    pub fn write_only(mut self, write_only: bool) -> Self {
        self.write_only = write_only;
        self
    }

    /// 设置 oneOf
    pub fn one_of(mut self, schemas: Vec<Schema>) -> Self {
        self.one_of = Some(schemas);
        self
    }

    /// 设置 anyOf
    pub fn any_of(mut self, schemas: Vec<Schema>) -> Self {
        self.any_of = Some(schemas);
        self
    }

    /// 设置 allOf
    pub fn all_of(mut self, schemas: Vec<Schema>) -> Self {
        self.all_of = Some(schemas);
        self
    }

    /// 转换为 JSON Schema
    pub fn to_json_schema(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}



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

/// OpenAPI 文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiDoc {
    /// OpenAPI 版本
    pub openapi: String,
    /// API 信息
    pub info: OpenApiInfo,
    /// 路径定义
    pub paths: std::collections::BTreeMap<String, PathItem>,
    /// 组件定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
    /// 服务器列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    /// 标签列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// 外部文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// 安全要求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
}

impl Default for OpenApiDoc {
    /// 创建默认的 OpenAPI 文档
    fn default() -> Self {
        Self {
            openapi: "3.1.0".to_string(),
            info: OpenApiInfo::default(),
            paths: std::collections::BTreeMap::new(),
            components: None,
            servers: None,
            tags: None,
            external_docs: None,
            security: None,
        }
    }
}

impl OpenApiDoc {
    /// 创建新的 OpenAPI 文档
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self { info: OpenApiInfo { title: title.into(), version: version.into(), ..Default::default() }, ..Default::default() }
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.info.description = Some(desc.into());
        self
    }

    /// 添加路径
    pub fn path(mut self, path: impl Into<String>, item: PathItem) -> Self {
        self.paths.insert(path.into(), item);
        self
    }

    /// 添加 Schema 组件
    pub fn schema(mut self, name: impl Into<String>, schema: Schema) -> Self {
        let components = self.components.get_or_insert_with(Components::default);
        components.schemas.insert(name.into(), schema);
        self
    }

    /// 添加安全方案组件
    pub fn security_scheme(mut self, name: impl Into<String>, scheme: SecurityScheme) -> Self {
        let components = self.components.get_or_insert_with(Components::default);
        components.security_schemes.insert(name.into(), scheme);
        self
    }

    /// 添加响应组件
    pub fn response(mut self, name: impl Into<String>, response: Response) -> Self {
        let components = self.components.get_or_insert_with(Components::default);
        components.responses.insert(name.into(), response);
        self
    }

    /// 添加参数组件
    pub fn parameter(mut self, name: impl Into<String>, parameter: Parameter) -> Self {
        let components = self.components.get_or_insert_with(Components::default);
        components.parameters.insert(name.into(), parameter);
        self
    }

    /// 添加请求体组件
    pub fn request_body(mut self, name: impl Into<String>, body: RequestBody) -> Self {
        let components = self.components.get_or_insert_with(Components::default);
        components.request_bodies.insert(name.into(), body);
        self
    }

    /// 添加服务器
    pub fn server(mut self, url: impl Into<String>, description: Option<String>) -> Self {
        let servers = self.servers.get_or_insert_with(Vec::new);
        servers.push(Server { url: url.into(), description, variables: None });
        self
    }

    /// 添加标签
    pub fn tag(mut self, name: impl Into<String>, description: Option<String>) -> Self {
        let tags = self.tags.get_or_insert_with(Vec::new);
        tags.push(Tag { name: name.into(), description, external_docs: None });
        self
    }

    /// 设置外部文档
    pub fn external_docs(mut self, url: impl Into<String>, description: Option<String>) -> Self {
        self.external_docs = Some(ExternalDocumentation { url: url.into(), description });
        self
    }

    /// 设置安全要求
    pub fn security(mut self, security: SecurityRequirement) -> Self {
        let securities = self.security.get_or_insert_with(Vec::new);
        securities.push(security);
        self
    }

    /// 转换为 JSON
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    /// 转换为 YAML
    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap_or_default()
    }
}

/// 路径项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    /// GET 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    /// POST 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    /// PUT 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    /// DELETE 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    /// PATCH 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    /// HEAD 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    /// OPTIONS 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    /// TRACE 操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,
    /// 路径描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 路径摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 路径参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterOrReference>>,
    /// 服务器列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
}

impl Default for PathItem {
    /// 创建默认的路径项
    fn default() -> Self {
        Self {
            get: None,
            post: None,
            put: None,
            delete: None,
            patch: None,
            head: None,
            options: None,
            trace: None,
            description: None,
            summary: None,
            parameters: None,
            servers: None,
        }
    }
}

impl PathItem {
    /// 创建新的路径项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 GET 操作
    pub fn get(mut self, op: Operation) -> Self {
        self.get = Some(op);
        self
    }

    /// 设置 POST 操作
    pub fn post(mut self, op: Operation) -> Self {
        self.post = Some(op);
        self
    }

    /// 设置 PUT 操作
    pub fn put(mut self, op: Operation) -> Self {
        self.put = Some(op);
        self
    }

    /// 设置 DELETE 操作
    pub fn delete(mut self, op: Operation) -> Self {
        self.delete = Some(op);
        self
    }

    /// 设置 PATCH 操作
    pub fn patch(mut self, op: Operation) -> Self {
        self.patch = Some(op);
        self
    }

    /// 设置 HEAD 操作
    pub fn head(mut self, op: Operation) -> Self {
        self.head = Some(op);
        self
    }

    /// 设置 OPTIONS 操作
    pub fn options(mut self, op: Operation) -> Self {
        self.options = Some(op);
        self
    }

    /// 设置 TRACE 操作
    pub fn trace(mut self, op: Operation) -> Self {
        self.trace = Some(op);
        self
    }
}

/// 操作定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// 操作标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 操作摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 操作描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 操作 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// 参数列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterOrReference>>,
    /// 请求体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBodyOrReference>,
    /// 响应定义
    pub responses: std::collections::BTreeMap<String, ResponseOrReference>,
    /// 回调定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<std::collections::BTreeMap<String, Callback>>,
    /// 弃用标记
    #[serde(default)]
    pub deprecated: bool,
    /// 安全要求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirement>>,
    /// 服务器列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<Server>>,
    /// 外部文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

impl Operation {
    /// 创建新操作
    pub fn new() -> Self {
        Self {
            tags: None,
            summary: None,
            description: None,
            operation_id: None,
            parameters: None,
            request_body: None,
            responses: std::collections::BTreeMap::new(),
            callbacks: None,
            deprecated: false,
            security: None,
            servers: None,
            external_docs: None,
        }
    }

    /// 设置摘要
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// 设置操作 ID
    pub fn operation_id(mut self, id: impl Into<String>) -> Self {
        self.operation_id = Some(id.into());
        self
    }

    /// 添加标签
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        let tags = self.tags.get_or_insert_with(Vec::new);
        tags.push(tag.into());
        self
    }

    /// 添加参数
    pub fn parameter(mut self, param: ParameterOrReference) -> Self {
        let params = self.parameters.get_or_insert_with(Vec::new);
        params.push(param);
        self
    }

    /// 设置请求体
    pub fn request_body(mut self, body: RequestBodyOrReference) -> Self {
        self.request_body = Some(body);
        self
    }

    /// 添加响应
    pub fn response(mut self, status: impl Into<String>, resp: ResponseOrReference) -> Self {
        self.responses.insert(status.into(), resp);
        self
    }

    /// 设置弃用
    pub fn deprecated(mut self, deprecated: bool) -> Self {
        self.deprecated = deprecated;
        self
    }
}

impl Default for Operation {
    /// 创建默认的操作
    fn default() -> Self {
        Self::new()
    }
}

/// 参数定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// 参数名称
    pub name: String,
    /// 参数位置
    #[serde(rename = "in")]
    pub location: ParameterLocation,
    /// 参数描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必需
    #[serde(default)]
    pub required: bool,
    /// 参数 Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// 是否废弃
    #[serde(default)]
    pub deprecated: bool,
    /// 是否允许空值
    #[serde(default)]
    pub allow_empty_value: bool,
    /// 示例值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// 多个示例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<std::collections::BTreeMap<String, ExampleOrReference>>,
}

/// 参数位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    /// 路径参数
    Path,
    /// 查询参数
    Query,
    /// 请求头参数
    Header,
    /// Cookie 参数
    Cookie,
}

impl Parameter {
    /// 创建路径参数
    pub fn path(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: ParameterLocation::Path,
            description: None,
            required: true,
            schema: None,
            deprecated: false,
            allow_empty_value: false,
            example: None,
            examples: None,
        }
    }

    /// 创建查询参数
    pub fn query(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: ParameterLocation::Query,
            description: None,
            required: false,
            schema: None,
            deprecated: false,
            allow_empty_value: false,
            example: None,
            examples: None,
        }
    }

    /// 创建请求头参数
    pub fn header(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: ParameterLocation::Header,
            description: None,
            required: false,
            schema: None,
            deprecated: false,
            allow_empty_value: false,
            example: None,
            examples: None,
        }
    }

    /// 创建 Cookie 参数
    pub fn cookie(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            location: ParameterLocation::Cookie,
            description: None,
            required: false,
            schema: None,
            deprecated: false,
            allow_empty_value: false,
            example: None,
            examples: None,
        }
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// 设置必需
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// 设置 Schema
    pub fn schema(mut self, schema: Schema) -> Self {
        self.schema = Some(schema);
        self
    }

    /// 设置废弃
    pub fn deprecated(mut self, deprecated: bool) -> Self {
        self.deprecated = deprecated;
        self
    }

    /// 设置允许空值
    pub fn allow_empty_value(mut self, allow: bool) -> Self {
        self.allow_empty_value = allow;
        self
    }

    /// 设置示例值
    pub fn example(mut self, example: serde_json::Value) -> Self {
        self.example = Some(example);
        self
    }
}

/// 引用对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// 引用地址
    #[serde(rename = "$ref")]
    pub ref_path: String,
}

/// 参数或引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterOrReference {
    /// 参数对象
    Parameter(Parameter),
    /// 引用对象
    Reference(Reference),
}

/// 请求体定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    /// 请求体描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 内容定义
    pub content: std::collections::BTreeMap<String, MediaType>,
    /// 是否必需
    #[serde(default)]
    pub required: bool,
}

impl RequestBody {
    /// 创建 JSON 请求体
    pub fn json(schema: Schema) -> Self {
        let mut content = std::collections::BTreeMap::new();
        content.insert(
            "application/json".to_string(),
            MediaType { schema: Some(schema), example: None, examples: None, encoding: None },
        );
        Self { description: None, content, required: true }
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// 设置是否必需
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

/// 请求体或引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestBodyOrReference {
    /// 请求体对象
    RequestBody(RequestBody),
    /// 引用对象
    Reference(Reference),
}

/// 媒体类型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    /// Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// 示例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// 多个示例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<std::collections::BTreeMap<String, ExampleOrReference>>,
    /// 编码定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<std::collections::BTreeMap<String, Encoding>>,
}

/// 编码定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encoding {
    /// 内容类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 请求头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::BTreeMap<String, HeaderOrReference>>,
    /// 样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// 是否展开
    #[serde(default)]
    pub explode: bool,
    /// 是否允许保留字符
    #[serde(default)]
    pub allow_reserved: bool,
}

/// 响应定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// 响应描述
    pub description: String,
    /// 内容定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::BTreeMap<String, MediaType>>,
    /// 响应头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::BTreeMap<String, HeaderOrReference>>,
    /// 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::BTreeMap<String, LinkOrReference>>,
}

impl Response {
    /// 创建新响应
    pub fn new(description: impl Into<String>) -> Self {
        Self { description: description.into(), content: None, headers: None, links: None }
    }

    /// 添加 JSON 响应
    pub fn json(mut self, schema: Schema) -> Self {
        let content = self.content.get_or_insert_with(std::collections::BTreeMap::new);
        content.insert(
            "application/json".to_string(),
            MediaType { schema: Some(schema), example: None, examples: None, encoding: None },
        );
        self
    }
}

/// 响应或引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseOrReference {
    /// 响应对象
    Response(Response),
    /// 引用对象
    Reference(Reference),
}

/// 头部定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// 头部描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必需
    #[serde(default)]
    pub required: bool,
    /// Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    /// 是否废弃
    #[serde(default)]
    pub deprecated: bool,
    /// 示例值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    /// 多个示例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<std::collections::BTreeMap<String, ExampleOrReference>>,
}

/// 头部或引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeaderOrReference {
    /// 头部对象
    Header(Header),
    /// 引用对象
    Reference(Reference),
}

/// 示例定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    /// 示例摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 示例描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 示例值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// 外部示例值 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}

/// 示例或引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExampleOrReference {
    /// 示例对象
    Example(Example),
    /// 引用对象
    Reference(Reference),
}

/// 链接定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// 链接操作引用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
    /// 链接操作 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// 链接参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// 链接请求体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<serde_json::Value>,
    /// 链接描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 链接服务器
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

/// 链接或引用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkOrReference {
    /// 链接对象
    Link(Link),
    /// 引用对象
    Reference(Reference),
}

/// 回调定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callback {
    /// 回调路径项
    #[serde(flatten)]
    pub paths: std::collections::BTreeMap<String, PathItem>,
}

/// 安全要求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityRequirement {
    /// 安全方案名称和范围
    #[serde(flatten)]
    pub schemes: std::collections::BTreeMap<String, Vec<String>>,
}

impl SecurityRequirement {
    /// 创建新的安全要求
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加安全方案
    pub fn scheme(mut self, name: impl Into<String>, scopes: Vec<&str>) -> Self {
        self.schemes.insert(name.into(), scopes.into_iter().map(String::from).collect());
        self
    }
}

/// 安全方案类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SecuritySchemeType {
    /// API Key
    ApiKey,
    /// HTTP 认证
    Http,
    /// OAuth2
    OAuth2,
    /// OpenID Connect
    OpenIdConnect,
}

/// API Key 位置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyLocation {
    /// 查询参数
    Query,
    /// 请求头
    Header,
    /// Cookie
    Cookie,
}

/// 安全方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScheme {
    /// 安全方案类型
    #[serde(rename = "type")]
    pub scheme_type: SecuritySchemeType,
    /// 安全方案描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// API Key 名称（仅 apiKey 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// API Key 位置（仅 apiKey 类型）
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ApiKeyLocation>,
    /// HTTP 方案名称（仅 http 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// HTTP Bearer 格式（仅 http 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    /// OAuth2 流程（仅 oauth2 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<OAuthFlows>,
    /// OpenID Connect URL（仅 openIdConnect 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_url: Option<String>,
}

impl SecurityScheme {
    /// 创建 API Key 安全方案
    pub fn api_key(name: impl Into<String>, location: ApiKeyLocation) -> Self {
        Self {
            scheme_type: SecuritySchemeType::ApiKey,
            description: None,
            name: Some(name.into()),
            location: Some(location),
            scheme: None,
            bearer_format: None,
            flows: None,
            open_id_connect_url: None,
        }
    }

    /// 创建 HTTP Basic 安全方案
    pub fn basic() -> Self {
        Self {
            scheme_type: SecuritySchemeType::Http,
            description: None,
            name: None,
            location: None,
            scheme: Some("basic".to_string()),
            bearer_format: None,
            flows: None,
            open_id_connect_url: None,
        }
    }

    /// 创建 HTTP Bearer 安全方案
    pub fn bearer(format: Option<String>) -> Self {
        Self {
            scheme_type: SecuritySchemeType::Http,
            description: None,
            name: None,
            location: None,
            scheme: Some("bearer".to_string()),
            bearer_format: format,
            flows: None,
            open_id_connect_url: None,
        }
    }

    /// 创建 OAuth2 安全方案
    pub fn oauth2(flows: OAuthFlows) -> Self {
        Self {
            scheme_type: SecuritySchemeType::OAuth2,
            description: None,
            name: None,
            location: None,
            scheme: None,
            bearer_format: None,
            flows: Some(flows),
            open_id_connect_url: None,
        }
    }

    /// 创建 OpenID Connect 安全方案
    pub fn open_id_connect(url: impl Into<String>) -> Self {
        Self {
            scheme_type: SecuritySchemeType::OpenIdConnect,
            description: None,
            name: None,
            location: None,
            scheme: None,
            bearer_format: None,
            flows: None,
            open_id_connect_url: Some(url.into()),
        }
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// OAuth2 流程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlows {
    /// Implicit 流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlow>,
    /// Password 流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlow>,
    /// Client Credentials 流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlow>,
    /// Authorization Code 流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlow>,
}

/// OAuth2 流程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    /// 授权 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    /// Token URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    /// 刷新 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// 可用的范围
    pub scopes: std::collections::BTreeMap<String, String>,
}

impl OAuthFlow {
    /// 创建新的 OAuth 流程
    pub fn new() -> Self {
        Self { authorization_url: None, token_url: None, refresh_url: None, scopes: std::collections::BTreeMap::new() }
    }

    /// 设置授权 URL
    pub fn authorization_url(mut self, url: impl Into<String>) -> Self {
        self.authorization_url = Some(url.into());
        self
    }

    /// 设置 Token URL
    pub fn token_url(mut self, url: impl Into<String>) -> Self {
        self.token_url = Some(url.into());
        self
    }

    /// 设置刷新 URL
    pub fn refresh_url(mut self, url: impl Into<String>) -> Self {
        self.refresh_url = Some(url.into());
        self
    }

    /// 添加范围
    pub fn scope(mut self, name: impl Into<String>, description: impl Into<String>) -> Self {
        self.scopes.insert(name.into(), description.into());
        self
    }
}

impl Default for OAuthFlow {
    fn default() -> Self {
        Self::new()
    }
}

/// 组件定义
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Components {
    /// Schema 定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub schemas: std::collections::BTreeMap<String, Schema>,
    /// 响应定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub responses: std::collections::BTreeMap<String, Response>,
    /// 参数定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub parameters: std::collections::BTreeMap<String, Parameter>,
    /// 示例定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub examples: std::collections::BTreeMap<String, Example>,
    /// 请求体定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub request_bodies: std::collections::BTreeMap<String, RequestBody>,
    /// 头部定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub headers: std::collections::BTreeMap<String, Header>,
    /// 安全方案定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub security_schemes: std::collections::BTreeMap<String, SecurityScheme>,
    /// 链接定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub links: std::collections::BTreeMap<String, Link>,
    /// 回调定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub callbacks: std::collections::BTreeMap<String, Callback>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ToSchema for String {
    fn schema() -> Schema {
        Schema::string()
    }
}

impl ToSchema for i64 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for i32 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for i16 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for i8 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u64 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u32 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u16 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for u8 {
    fn schema() -> Schema {
        Schema::integer()
    }
}

impl ToSchema for f64 {
    fn schema() -> Schema {
        Schema::number()
    }
}

impl ToSchema for f32 {
    fn schema() -> Schema {
        Schema::number()
    }
}

impl ToSchema for bool {
    fn schema() -> Schema {
        Schema::boolean()
    }
}

impl<T: ToSchema> ToSchema for Vec<T> {
    fn schema() -> Schema {
        Schema::array(T::schema())
    }
}

impl<T: ToSchema> ToSchema for Option<T> {
    fn schema() -> Schema {
        T::schema().nullable(true)
    }
}

impl ToSchema for serde_json::Value {
    fn schema() -> Schema {
        <Schema as Default>::default()
    }
}
