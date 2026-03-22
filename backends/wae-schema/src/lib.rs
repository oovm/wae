//! WAE Schema - Schema 定义与验证模块
//! 
//! 提供统一的 Schema 定义能力，支持：
//! - 数据结构 Schema 定义
//! - Schema 验证
//! - OpenAPI 文档生成
#![warn(missing_docs)]

/// 重新导出 itools-schema 的核心功能
pub use itools_schema::*;

/// OpenAPI 相关功能
pub mod openapi;
/// Swagger UI 相关功能
pub mod swagger_ui;