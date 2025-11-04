use axum::{Json, extract::Path, extract::Query, extract::State, http::StatusCode};
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use tracing::error;

use crate::{services::ohlc_service, api::models::{OhlcDateRangeRequest, OhlcResponse}};

/// 获取所有OHLC数据
#[axum::debug_handler]
pub async fn get_all_ohlc(
    db: State<DatabaseConnection>,
) -> Result<Json<OhlcResponse>, StatusCode> {
    match ohlc_service::get_all_ohlc_data(&db).await {
        Ok(data) => Ok(Json(OhlcResponse { data })),
        Err(e) => {
            error!("Failed to get all OHLC data: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 根据交易对获取OHLC数据
#[axum::debug_handler]
pub async fn get_ohlc_by_symbol(
    Path(symbol): Path<String>,
    db: State<DatabaseConnection>,
) -> Result<Json<OhlcResponse>, StatusCode> {
    match ohlc_service::get_ohlc_data_by_code(&db, &symbol).await {
        Ok(data) => Ok(Json(OhlcResponse { data })),
        Err(e) => {
            error!("Failed to get OHLC data for symbol {}: {}", symbol, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 根据交易对和时间范围获取OHLC数据
#[axum::debug_handler]
pub async fn get_ohlc_by_date_range(
    Path(symbol): Path<String>,
    Query(params): Query<OhlcDateRangeRequest>,
    db: State<DatabaseConnection>,
) -> Result<Json<OhlcResponse>, StatusCode> {
    // 解析时间参数
    let start_date = NaiveDateTime::parse_from_str(&params.start, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let end_date = NaiveDateTime::parse_from_str(&params.end, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 查询数据库
    match ohlc_service::get_ohlc_data_by_date_range(&db, &symbol, start_date, end_date).await {
        Ok(data) => Ok(Json(OhlcResponse { data })),
        Err(e) => {
            error!(
                "Failed to get OHLC data for symbol {} in range {} to {}: {}",
                symbol,
                params.start,
                params.end,
                e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}