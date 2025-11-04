use axum::{Router, routing::get};
use migration::Migrator;
use sea_orm::*;
use sea_orm_migration::migrator::MigratorTrait;
use std::net::SocketAddr;

// 导入API层模块
mod api;
use api::ohlc_api;

mod entity;
mod migration;
mod services;

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
        .route("/api/ohlc", get(ohlc_api::get_all_ohlc))
        .route("/api/ohlc/:symbol", get(ohlc_api::get_ohlc_by_symbol))
        .route(
            "/api/ohlc/:symbol/range",
            get(ohlc_api::get_ohlc_by_date_range),
        )
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
