use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

mod app;
mod config;
mod handlers;
mod listener;
mod logger;
mod models;
mod routes;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // NOTE: early dev
    dotenv::dotenv().ok();
    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();
    logger::setup();
    // get proper listener
    let listener = listener::try_create().await?;
    tracing::info!("listening on {}", listener.local_addr()?);

    // set up connection pool
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            tracing::debug!("✅ Connected to database!");
            pool
        }
        Err(err) => {
            tracing::info!("🔥 connection to the database failed: {:?}", err);
            std::process::exit(1);
        }
    };

    sqlx::migrate!().run(&pool).await?;

    let app = app::create(pool);
    Ok(axum::serve(listener, app).await?)
}
