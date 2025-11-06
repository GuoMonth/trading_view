use crate::entity::ohlc_data::Entity as OHLCData;
use crate::entity::ohlc_data;
use sea_orm::*;
use chrono::NaiveDateTime;

pub async fn get_all_ohlc_data(db: &DatabaseConnection) -> Result<Vec<ohlc_data::Model>, DbErr> {
    OHLCData::find().all(db).await
}

pub async fn get_ohlc_data_by_code(db: &DatabaseConnection, code: &str) -> Result<Vec<ohlc_data::Model>, DbErr> {
    OHLCData::find()
        .filter(ohlc_data::Column::Code.eq(code))
        .all(db)
        .await
}

pub async fn get_ohlc_data_by_date_range(
    db: &DatabaseConnection,
    code: &str,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<Vec<ohlc_data::Model>, DbErr> {
    OHLCData::find()
        .filter(ohlc_data::Column::Code.eq(code))
        .filter(ohlc_data::Column::Timestamp.between(start_date, end_date))
        .order_by_asc(ohlc_data::Column::Timestamp)
        .all(db)
        .await
}