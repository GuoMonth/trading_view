use axum::{
    Json, Router, extract::Path, extract::Query, extract::State, http::StatusCode, routing::get,
};
use chrono::NaiveDateTime;
use migration::Migrator;
use sea_orm::*;
use sea_orm_migration::migrator::MigratorTrait;
use serde::Deserialize;
use std::net::SocketAddr;

/// 时间范围查询参数
#[derive(Deserialize, Debug)]
pub struct DateRangeParams {
    pub start: String,
    pub end: String,
}

/// 根据交易对和时间范围获取OHLC数据
#[axum::debug_handler]
async fn get_ohlc_by_date_range(
    Path(symbol): Path<String>,
    Query(params): Query<DateRangeParams>,
    db: State<DatabaseConnection>,
) -> Result<Json<Vec<entity::ohlc_data::Model>>, StatusCode> {
    // 解析时间参数
    let start_date = NaiveDateTime::parse_from_str(&params.start, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let end_date = NaiveDateTime::parse_from_str(&params.end, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 查询数据库
    match ohlc_service::get_ohlc_data_by_date_range(&db, &symbol, start_date, end_date).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            tracing::error!(
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

mod entity;
mod migration;
mod services;
use services::ohlc_service;

#[tokio::main]
async fn main() {
    // 配置日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 创建数据库连接
    let db = match establish_connection().await {
        Ok(db) => db,
        Err(e) => {
            tracing::error!("Failed to establish database connection: {}", e);
            return;
        }
    };

    // 执行数据库迁移
    if let Err(e) = Migrator::up(&db, None).await {
        tracing::error!("Failed to run database migrations: {}", e);
        return;
    }

    // 创建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/ohlc", get(get_all_ohlc))
        .route("/api/ohlc/:symbol", get(get_ohlc_by_symbol))
        .route("/api/ohlc/:symbol/range", get(get_ohlc_by_date_range))
        .with_state(db);

    // 监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    // 启动服务器
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

/// 建立数据库连接
async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    // 使用SQLite数据库文件
    let current_dir = std::env::current_dir()
        .map_err(|e| DbErr::Custom(format!("Failed to get current directory: {}", e)))?;
    let db_dir = current_dir.join("db");
    let db_path = db_dir.join("trading_view.db");
    let db_url = format!(
        "sqlite://{}",
        db_path
            .to_str()
            .ok_or_else(|| DbErr::Custom("Invalid database path".to_string()))?
    );

    tracing::info!("Connecting to database: {}", db_url);
    Database::connect(db_url).await
}

/// 健康检查处理函数
async fn health_check() -> &'static str {
    "OK"
}

/// 获取所有OHLC数据
async fn get_all_ohlc(
    db: State<DatabaseConnection>,
) -> Result<Json<Vec<entity::ohlc_data::Model>>, StatusCode> {
    match ohlc_service::get_all_ohlc_data(&db).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            tracing::error!("Failed to get all OHLC data: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 根据交易对获取OHLC数据
async fn get_ohlc_by_symbol(
    Path(symbol): Path<String>,
    db: State<DatabaseConnection>,
) -> Result<Json<Vec<entity::ohlc_data::Model>>, StatusCode> {
    match ohlc_service::get_ohlc_data_by_code(&db, &symbol).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            tracing::error!("Failed to get OHLC data for symbol {}: {}", symbol, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 时间范围查询参数
#[derive(Deserialize, Debug)]
pub struct DateRangeParams {
    pub start: String,
    pub end: String,
}
