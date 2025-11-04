use sea_orm::entity::prelude::*;
use sea_orm_migration::seaql_migrations::Relation;
use sea_orm::Set;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "ohlc_data")]
#[sea_orm(unique_key = ("code", "timestamp"))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub code: String,
    pub timestamp: NaiveDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub created_at: NaiveDateTime,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            created_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }
}