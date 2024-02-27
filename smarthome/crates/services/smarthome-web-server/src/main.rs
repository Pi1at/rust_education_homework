use axum::BoxError;

mod app;
mod listener;
mod logger;
mod routes;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    // build our application with a route
    let app = app::create();
    // get proper listener
    let listener = listener::try_create().await?;
    tracing::debug!("listening on {}", listener.local_addr()?);

    // run it
    axum::serve(listener, app).await?;
    Ok(())
}
