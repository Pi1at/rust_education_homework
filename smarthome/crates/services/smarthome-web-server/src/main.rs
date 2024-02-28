use anyhow::Ok;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
mod app;
mod config;
mod listener;
mod logger;
mod routes;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // NOTE: early dev
    dotenv::dotenv().ok();
    // build our application with a route
    let app = app::create();
    // get proper listener
    let listener = listener::try_create().await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // set up connection pool
    let db = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&db).await?;
    // run it
    axum::serve(listener, app).await?;
    Ok(())
}
