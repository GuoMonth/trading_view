//! Migration module for the trading view application

mod _20240520_000001_create_tables;

pub use sea_orm_migration::prelude::*;
use sea_orm_migration::MigratorTrait;

// Alias for the migration file
pub use self::_20240520_000001_create_tables as create_tables;

#[derive(Clone, Debug, DeriveMigrator)]
#[migrator(
    migrations = [
        create_tables::Migration,
    ]
)]
pub struct Migrator;