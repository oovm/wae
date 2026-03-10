//! 数据库连接模块

mod config;
mod row;
mod statement;
mod trait_impl;
mod value_impl;

#[cfg(feature = "turso")]
mod turso;

#[cfg(feature = "postgres")]
pub(crate) mod postgres;

#[cfg(feature = "mysql")]
mod mysql;

pub use config::{DatabaseConfig, DatabaseError, DatabaseResult};
pub use row::{DatabaseRow, DatabaseRows};
pub use statement::DatabaseStatement;
pub use trait_impl::{DatabaseConnection, DatabaseBackend};
pub use value_impl::FromDatabaseValue;

#[cfg(feature = "turso")]
pub use turso::{DatabaseService, TursoConnection};

#[cfg(feature = "postgres")]
pub use postgres::{PostgresConnection, PostgresDatabaseService};

#[cfg(feature = "mysql")]
pub use mysql::{MySqlConnection, MySqlDatabaseService};
