use sea_orm::entity::prelude::*;
use sea_orm::Set;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "symbol")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub symbol: String,
    pub name: String,
    pub symbol_type: String,
    pub exchange: String,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub lot_size: Option<f64>,
    pub tick_size: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ohlc_data::Entity")]
    OHLCData,
    #[sea_orm(has_many = "super::indicator_data::Entity")]
    IndicatorData,
    #[sea_orm(has_many = "super::trading_signal::Entity")]
    TradingSignal,
    #[sea_orm(has_many = "super::backtest_result::Entity")]
    BacktestResult,
}

impl Related<super::ohlc_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OHLCData.def()
    }
}

impl Related<super::indicator_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IndicatorData.def()
    }
}

impl Related<super::trading_signal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TradingSignal.def()
    }
}

impl Related<super::backtest_result::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BacktestResult.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            created_at: Set(now),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }

    fn update(mut self) -> Self {
        self.updated_at = Set(chrono::Utc::now().naive_utc());
        self
    }
}