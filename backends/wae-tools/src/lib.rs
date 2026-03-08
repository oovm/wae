//! WAE Tools - 开发工具集
//!
//! 提供数据库迁移、种子数据等开发辅助工具。
#![warn(missing_docs)]

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
pub mod auto_migrate;
#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
pub mod migration;

#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
pub use auto_migrate::{
    AutoMigrate, AutoMigrator, AutoMigratorConfig, ColumnDiff, DiffAction, IndexDiff, MigrationPlan, SchemaDiff,
    SchemaReflector, TableDiff,
};
#[cfg(any(feature = "database-turso", feature = "database-postgres", feature = "database-mysql"))]
pub use migration::{Migration, MigrationResult, Migrator, SimpleMigration};
