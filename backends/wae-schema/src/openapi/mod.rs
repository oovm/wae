//! OpenAPI 文档生成模块
//!
//! 提供 OpenAPI 文档构建功能。

#![warn(missing_docs)]

pub mod builder;

pub use builder::OpenApiBuilder;
