#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
mod connection;
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
mod orm;
mod schema;
#[cfg(feature = "turso")]
mod types;

#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
pub use connection::{
    DatabaseConfig, DatabaseConnection, DatabaseError, DatabaseResult, DatabaseRow, DatabaseRows, DatabaseStatement,
    FromDatabaseValue,
};
#[cfg(feature = "turso")]
pub use connection::{DatabaseService, TursoConnection};
#[cfg(feature = "mysql")]
pub use connection::{MySqlConnection, MySqlDatabaseService};
#[cfg(feature = "postgres")]
pub use connection::{PostgresConnection, PostgresDatabaseService};
#[cfg(feature = "turso")]
pub use orm::DbRepository;
#[cfg(feature = "mysql")]
pub use orm::MySqlDbRepository;
#[cfg(any(feature = "turso", feature = "postgres", feature = "mysql"))]
pub use orm::{
    Condition, DeleteBuilder, Entity, FromRow, InsertBuilder, QueryBuilder, Repository, SelectBuilder, ToRow, UpdateBuilder,
};
pub use schema::{ColumnDef, ColumnType, ForeignKeyDef, IndexDef, ReferentialAction, TableSchema, col};
pub use wae_types::Value;
