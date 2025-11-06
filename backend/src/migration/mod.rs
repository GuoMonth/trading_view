//! Migration module for the trading view application

pub mod _20240520_000001_create_tables;

pub use sea_orm_migration::prelude::*;
use sea_orm_migration::MigratorTrait;

// Alias for the migration file
pub use self::_20240520_000001_create_tables as create_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_tables::Migration),
        ]
    }
}