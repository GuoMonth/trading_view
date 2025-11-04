use sea_orm::entity::prelude::*;
use sea_orm::Set;
use chrono::NaiveDateTime;
use super::backtest_result::Entity as BacktestResult;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "trade_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub backtest_result_id: String,
    pub timestamp: NaiveDateTime,
    pub trade_type: String,
    pub price: f64,
    pub quantity: f64,
    pub amount: f64,
    pub fee: f64,
    pub remaining_capital: f64,
    pub position: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "BacktestResult", from = "Column::BacktestResultId", to = "super::backtest_result::Column::Id")]
    BacktestResult,
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