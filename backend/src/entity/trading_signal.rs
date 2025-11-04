use sea_orm::entity::prelude::*;
use sea_orm::Set;
use chrono::NaiveDateTime;
use super::symbol::Entity as Symbol;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "trading_signal")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub symbol_id: i32,
    pub timestamp: NaiveDateTime,
    pub signal_type: String,
    pub source: String,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "Symbol", from = "Column::SymbolId", to = "super::symbol::Column::Id")]
    Symbol,
}

impl Related<super::symbol::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Symbol.def()
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