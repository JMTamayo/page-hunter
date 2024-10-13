use axum::{serve as axum_serve, Extension};
use log::info;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

mod api;
use api::router::{ApiRouter, ServiceRouter};

mod config;
use config::conf::Conf;
use config::logconfig::LogConfig;

mod db;
use db::handler::DbHandler;

mod docs;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Conf = Conf::get();
    let db_handler: DbHandler = DbHandler::new();

    LogConfig::init_logger(settings.get_server_config().get_log_level());

    info!("Initializing database connection and running migrations");

    let pool: PgPool = db_handler.get_pool(settings.get_database_config()).await?;
    db_handler.run_migrations(&pool).await?;

    info!("Database connection and migrations completed successfully");

    info!(
        "Starting service: {}",
        settings.get_server_config().get_name()
    );
    info!(
        "Listening on: {:?}",
        settings.get_server_config().get_service_address()
    );

    Ok(axum_serve(
        TcpListener::bind(settings.get_server_config().get_service_address()).await?,
        ApiRouter::new()
            .get_router()
            .layer(Extension(Arc::new(pool))),
    )
    .await?)
}
