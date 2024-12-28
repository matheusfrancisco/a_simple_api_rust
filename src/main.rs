use std::str::FromStr;
mod todo;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tracing::level_filters::LevelFilter;
//use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn init_tracing() {
    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory".to_string());

    let dbpool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::from_str(&db_connection_str)?.create_if_missing(true))
        .await
        .expect("can’t connect to database");

    sqlx::migrate!()
        .run(&dbpool)
        .await
        .expect("database migration failed");

    Ok(dbpool)
}

#[tokio::main]
async fn main() {
    init_tracing();
    let dbpool = init_dbpool().await.expect("Failed to create pool");
    //let router = create_router(dbpool).await;

    //let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    //axum::Server::bind(&bind_addr.parse().unwrap()) .serve(router.into_make_service()) .await .expect("unable to start server")
}
