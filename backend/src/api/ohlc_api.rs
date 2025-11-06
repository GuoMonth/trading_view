use axum::{Json, extract::{Path, Query, State}, http::StatusCode};
use chrono::NaiveDateTime;
use sea_orm::DatabaseConnection;
use anyhow::{Context, Result};
use crate::api::error::ApiErrorWrapper;

use crate::{services::ohlc_service, api::models::{OhlcDateRangeRequest, OhlcResponse, ApiResponse}};

/// 获取所有OHLC数据
#[axum::debug_handler]
pub async fn get_all_ohlc(
    db: State<DatabaseConnection>,
) -> Result<(StatusCode, Json<ApiResponse<OhlcResponse>>), ApiErrorWrapper> {
    let data = ohlc_service::get_all_ohlc_data(&db)
        .await
        .context("Failed to get all OHLC data")
        .map_err(ApiErrorWrapper::from)?;

    let response = ApiResponse::success(OhlcResponse { data });
    Ok((StatusCode::OK, Json(response)))
}

/// 根据交易对获取OHLC数据
#[axum::debug_handler]
pub async fn get_ohlc_by_symbol(
    Path(symbol): Path<String>,
    db: State<DatabaseConnection>,
) -> Result<(StatusCode, Json<ApiResponse<OhlcResponse>>), ApiErrorWrapper> {
    let data = ohlc_service::get_ohlc_data_by_code(&db, &symbol)
        .await
        .context(format!("Failed to get OHLC data for symbol {}", symbol))
        .map_err(ApiErrorWrapper::from)?;

    let response = ApiResponse::success(OhlcResponse { data });
    Ok((StatusCode::OK, Json(response)))
}

/// 根据交易对和时间范围获取OHLC数据
#[axum::debug_handler]
pub async fn get_ohlc_by_date_range(
    Path(symbol): Path<String>,
    Query(params): Query<OhlcDateRangeRequest>,
    db: State<DatabaseConnection>,
) -> Result<(StatusCode, Json<ApiResponse<OhlcResponse>>), ApiErrorWrapper> {
    // 解析时间参数
    let start_date = NaiveDateTime::parse_from_str(&params.start, "%Y-%m-%d %H:%M:%S")
        .with_context(|| format!("Invalid date format for start '{}'", params.start))
        .map_err(ApiErrorWrapper::from)?;
    let end_date = NaiveDateTime::parse_from_str(&params.end, "%Y-%m-%d %H:%M:%S")
        .with_context(|| format!("Invalid date format for end '{}'", params.end))
        .map_err(ApiErrorWrapper::from)?;

    // 查询数据库
    let data = ohlc_service::get_ohlc_data_by_date_range(&db, &symbol, start_date, end_date)
        .await
        .with_context(|| format!("Failed to get OHLC data for symbol {} in range {} to {}", symbol, params.start, params.end))
        .map_err(ApiErrorWrapper::from)?;

    let response = ApiResponse::success(OhlcResponse { data });
    Ok((StatusCode::OK, Json(response)))
}