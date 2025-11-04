use axum::{routing::get, Router};
use std::net::SocketAddr;
use sea_orm::*;
use migration::Migrator;
use sea_orm_migration::migrator::MigratorTrait;

mod entity;
mod migration;

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
    if let Err(e) = Migrator::run(&db).await {
        tracing::error!("Failed to run database migrations: {}", e);
        return;
    }

    // 创建路由
    let app = Router::new()
        .route("/health", get(health_check))
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
    let db_path = "trading_view.db";
    let db_url = format!("sqlite://{}", db_path);
    
    tracing::info!("Connecting to database: {}", db_url);
    Database::connect(db_url).await
}

/// 健康检查处理函数
async fn health_check() -> &'static str {
    "OK"
}
