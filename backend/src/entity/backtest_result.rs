use sea_orm::entity::prelude::*;
use sea_orm::Set;
use chrono::NaiveDateTime;
use super::symbol::Entity as Symbol;
use super::trade_record::Entity as TradeRecord;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "backtest_result")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub symbol_id: i32,
    pub timeframe: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub initial_capital: f64,
    pub final_capital: f64,
    pub total_return: f64,
    pub annual_return: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub trade_count: i32,
    pub winning_trades: i32,
    pub losing_trades: i32,
    pub win_rate: f64,
    pub average_win: f64,
    pub average_loss: f64,
    pub profit_factor: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "Symbol", from = "Column::SymbolId", to = "super::symbol::Column::Id")]
    Symbol,
    #[sea_orm(has_many = "TradeRecord")]
    TradeRecord,
}

impl Related<super::symbol::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Symbol.def()
    }
}

impl Related<super::trade_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TradeRecord.def()
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