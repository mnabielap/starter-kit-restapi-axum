#![allow(dead_code)] 
#![allow(unused_imports)]

use std::sync::Arc;
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod domain;
mod error;
mod handler;
mod middleware;
mod repository;
mod routes;
mod usecase;

use config::CONFIG;
use routes::create_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "starter_kit_restapi_axum=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&CONFIG.database_url)
        .await?;

    tracing::info!("🚀 Running database migrations...");
    sqlx::migrate!()
        .run(&pool)
        .await?;
    tracing::info!("✅ Migrations success");

    // 3. Wrap pool
    let db_pool = Arc::new(pool);
    tracing::info!("Connected to database");

    let app = create_router(db_pool);

    let listener = TcpListener::bind(format!("{}:{}", CONFIG.server_host, CONFIG.server_port)).await?;
    tracing::info!("Server listening on {}", listener.local_addr()?);
    axum::serve(listener, app.layer(TraceLayer::new_for_http())).await?;

    Ok(())
}