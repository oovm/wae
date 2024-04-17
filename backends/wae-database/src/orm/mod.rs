//! ORM 核心模块
//!
//! 提供轻量级 ORM 功能，支持：
//! - 实体定义和映射
//! - 查询构建器
//! - CRUD 操作封装

#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
mod builder;
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
mod condition;
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
mod entity;
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
mod macros;
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
mod repository;

#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
pub use builder::{DeleteBuilder, InsertBuilder, QueryBuilder, SelectBuilder, UpdateBuilder};
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
pub use condition::Condition;
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
pub use entity::{Entity, FromRow, ToRow};
#[cfg(feature = "mysql")]
pub use repository::MySqlDbRepository;
#[cfg(any(feature = "turso", feature = "postgres"))]
pub use repository::{DbRepository, Repository};
