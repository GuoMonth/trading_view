use axum::{Json, extract::Path, extract::Query, extract::State, http::StatusCode, response::{IntoResponse}};
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use tracing::error;

use crate::{services::ohlc_service, api::models::{OhlcDateRangeRequest, OhlcResponse, ApiResponse, ApiError}};

/// 获取所有OHLC数据
#[axum::debug_handler]
pub async fn get_all_ohlc(
    db: State<DatabaseConnection>,
) -> impl IntoResponse {
    match ohlc_service::get_all_ohlc_data(&db).await {
        Ok(data) => {
            let response = ApiResponse::success(OhlcResponse { data });
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let error_msg = format!("Failed to get all OHLC data: {}", e);
            error!("{}", error_msg);
            let response = ApiResponse::error_with_message(ApiError::DatabaseError, error_msg);
            (ApiError::DatabaseError.status_code(), Json(response))
        }
    }
}

/// 根据交易对获取OHLC数据
#[axum::debug_handler]
pub async fn get_ohlc_by_symbol(
    Path(symbol): Path<String>,
    db: State<DatabaseConnection>,
) -> impl IntoResponse {
    match ohlc_service::get_ohlc_data_by_code(&db, &symbol).await {
        Ok(data) => {
            let response = ApiResponse::success(OhlcResponse { data });
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let error_msg = format!("Failed to get OHLC data for symbol {}: {}", symbol, e);
            error!("{}", error_msg);
            let response = ApiResponse::error_with_message(ApiError::DatabaseError, error_msg);
            (ApiError::DatabaseError.status_code(), Json(response))
        }
    }
}

/// 根据交易对和时间范围获取OHLC数据
#[axum::debug_handler]
pub async fn get_ohlc_by_date_range(
    Path(symbol): Path<String>,
    Query(params): Query<OhlcDateRangeRequest>,
    db: State<DatabaseConnection>,
) -> impl IntoResponse {
    // 解析时间参数
    let start_date = match NaiveDateTime::parse_from_str(&params.start, "%Y-%m-%d %H:%M:%S") {
        Ok(date) => date,
        Err(e) => {
            let error_msg = format!("Invalid date format for start '{}': {}", params.start, e);
            error!("{}", error_msg);
            let response = ApiResponse::error_with_message(ApiError::InvalidDateFormat, error_msg);
            return (ApiError::InvalidDateFormat.status_code(), Json(response));
        }
    };
    let end_date = match NaiveDateTime::parse_from_str(&params.end, "%Y-%m-%d %H:%M:%S") {
        Ok(date) => date,
        Err(e) => {
            let error_msg = format!("Invalid date format for end '{}': {}", params.end, e);
            error!("{}", error_msg);
            let response = ApiResponse::error_with_message(ApiError::InvalidDateFormat, error_msg);
            return (ApiError::InvalidDateFormat.status_code(), Json(response));
        }
    };

    // 查询数据库
    match ohlc_service::get_ohlc_data_by_date_range(&db, &symbol, start_date, end_date).await {
        Ok(data) => {
            let response = ApiResponse::success(OhlcResponse { data });
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let error_msg = format!("Failed to get OHLC data for symbol {} in range {} to {}: {}", symbol, params.start, params.end, e);
            error!("{}", error_msg);
            let response = ApiResponse::error_with_message(ApiError::DatabaseError, error_msg);
            (ApiError::DatabaseError.status_code(), Json(response))
        }
    }
}