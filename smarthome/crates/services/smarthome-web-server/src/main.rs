mod app;
mod listener;
mod logger;
mod routes;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = app::create();
    // get proper listener
    let listener = listener::create().await;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // run it
    axum::serve(listener, app).await.unwrap();
}
