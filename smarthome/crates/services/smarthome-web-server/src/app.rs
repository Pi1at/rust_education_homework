use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Extension, Router,
};
use sqlx::PgPool;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{handlers, routes};

pub fn create(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health_check", get(handlers::health_check))
        .nest("/api", routes::location_schema())
        .layer(Extension(pool))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        // add a fallback service for handling routes to unknown paths
        .fallback(handlers::not_found_404)
}
