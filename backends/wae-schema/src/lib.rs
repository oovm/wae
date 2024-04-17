//! WAE Schema - Schema 定义与验证模块
//!
//! 提供统一的 Schema 定义能力，支持：
//! - 数据结构 Schema 定义
//! - Schema 验证
//! - OpenAPI 文档生成
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Schema 生成 trait
///
/// 实现此 trait 的类型可以自动生成 OpenAPI Schema。
pub trait ToSchema {
    /// 生成 Schema 定义
    fn schema() -> Schema;
}

/// Schema 类型定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    fn default() -> Self {
        Self::Object
    }
}

/// Schema 定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Schema 标题
    pub title: Option<String>,
    /// Schema 描述
    pub description: Option<String>,
    /// Schema 类型
    #[serde(rename = "type")]
    pub schema_type: SchemaType,
    /// 属性定义（仅 Object 类型）
    pub properties: Option<std::collections::BTreeMap<String, Schema>>,
    /// 必需属性列表
    pub required: Option<Vec<String>>,
    /// 数组元素类型（仅 Array 类型）
    pub items: Option<Box<Schema>>,
    /// 枚举值
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<serde_json::Value>>,
    /// 默认值
    pub default: Option<serde_json::Value>,
    /// 示例值
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
    pub minimum: Option<f64>,
    /// 最大值（Number/Integer）
    pub maximum: Option<f64>,
    /// 最小长度（String）
    pub min_length: Option<usize>,
    /// 最大长度（String）
    pub max_length: Option<usize>,
    /// 正则模式（String）
    pub pattern: Option<String>,
    /// 格式（String）
    pub format: Option<String>,
}

impl Default for Schema {
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

    /// 转换为 JSON Schema
    pub fn to_json_schema(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

/// Schema 构建器
pub struct SchemaBuilder {
    schema: Schema,
}

impl SchemaBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { schema: <Schema as Default>::default() }
    }

    /// 创建字符串构建器
    pub fn string() -> Self {
        Self { schema: Schema::string() }
    }

    /// 创建整数构建器
    pub fn integer() -> Self {
        Self { schema: Schema::integer() }
    }

    /// 创建数字构建器
    pub fn number() -> Self {
        Self { schema: Schema::number() }
    }

    /// 创建布尔构建器
    pub fn boolean() -> Self {
        Self { schema: Schema::boolean() }
    }

    /// 创建数组构建器
    pub fn array(items: Schema) -> Self {
        Self { schema: Schema::array(items) }
    }

    /// 创建对象构建器
    pub fn object() -> Self {
        Self { schema: Schema::object() }
    }

    /// 设置标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.schema.title = Some(title.into());
        self
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.schema.description = Some(desc.into());
        self
    }

    /// 添加属性
    pub fn property(mut self, name: impl Into<String>, schema: Schema) -> Self {
        let properties = self.schema.properties.get_or_insert_with(std::collections::BTreeMap::new);
        properties.insert(name.into(), schema);
        self
    }

    /// 设置必需属性
    pub fn required(mut self, fields: Vec<&str>) -> Self {
        self.schema.required = Some(fields.into_iter().map(String::from).collect());
        self
    }

    /// 构建 Schema
    pub fn build(self) -> Schema {
        self.schema
    }
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
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
    pub description: Option<String>,
    /// 服务条款 URL
    pub terms_of_service: Option<String>,
    /// 联系信息
    pub contact: Option<Contact>,
    /// 许可信息
    pub license: Option<License>,
}

impl Default for OpenApiInfo {
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
    pub name: Option<String>,
    /// 联系人邮箱
    pub email: Option<String>,
    /// 联系人 URL
    pub url: Option<String>,
}

/// 许可信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// 许可证名称
    pub name: String,
    /// 许可证 URL
    pub url: Option<String>,
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
    pub components: Option<Components>,
    /// 服务器列表
    pub servers: Option<Vec<Server>>,
}

impl Default for OpenApiDoc {
    fn default() -> Self {
        Self {
            openapi: "3.1.0".to_string(),
            info: OpenApiInfo::default(),
            paths: std::collections::BTreeMap::new(),
            components: None,
            servers: None,
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

    /// 添加服务器
    pub fn server(mut self, url: impl Into<String>, description: Option<String>) -> Self {
        let servers = self.servers.get_or_insert_with(Vec::new);
        servers.push(Server { url: url.into(), description });
        self
    }

    /// 转换为 JSON
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    /// 转换为 YAML（需要启用 yaml feature）
    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
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
    /// 路径描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 路径摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl Default for PathItem {
    fn default() -> Self {
        Self { get: None, post: None, put: None, delete: None, patch: None, description: None, summary: None }
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
    pub parameters: Option<Vec<Parameter>>,
    /// 请求体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,
    /// 响应定义
    pub responses: std::collections::BTreeMap<String, Response>,
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
    pub fn parameter(mut self, param: Parameter) -> Self {
        let params = self.parameters.get_or_insert_with(Vec::new);
        params.push(param);
        self
    }

    /// 设置请求体
    pub fn request_body(mut self, body: RequestBody) -> Self {
        self.request_body = Some(body);
        self
    }

    /// 添加响应
    pub fn response(mut self, status: impl Into<String>, resp: Response) -> Self {
        self.responses.insert(status.into(), resp);
        self
    }
}

impl Default for Operation {
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
        Self { name: name.into(), location: ParameterLocation::Path, description: None, required: true, schema: None }
    }

    /// 创建查询参数
    pub fn query(name: impl Into<String>) -> Self {
        Self { name: name.into(), location: ParameterLocation::Query, description: None, required: false, schema: None }
    }

    /// 创建请求头参数
    pub fn header(name: impl Into<String>) -> Self {
        Self { name: name.into(), location: ParameterLocation::Header, description: None, required: false, schema: None }
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
        content.insert("application/json".to_string(), MediaType { schema: Some(schema), example: None });
        Self { description: None, content, required: true }
    }

    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
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
}

/// 响应定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// 响应描述
    pub description: String,
    /// 内容定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::BTreeMap<String, MediaType>>,
}

impl Response {
    /// 创建新响应
    pub fn new(description: impl Into<String>) -> Self {
        Self { description: description.into(), content: None }
    }

    /// 添加 JSON 响应
    pub fn json(mut self, schema: Schema) -> Self {
        let content = self.content.get_or_insert_with(std::collections::BTreeMap::new);
        content.insert("application/json".to_string(), MediaType { schema: Some(schema), example: None });
        self
    }
}

/// 组件定义
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Components {
    /// Schema 定义
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub schemas: std::collections::BTreeMap<String, Schema>,
}

/// 服务器定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// 服务器 URL
    pub url: String,
    /// 服务器描述
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
