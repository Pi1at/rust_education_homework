use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::logger;

pub fn create() -> Router {
    logger::setup();
    Router::new()
        .route("/", get(crate::routes::handler))
        .route("/health_check", get(crate::routes::health_check))
        .route("/api/locations", get(crate::routes::get_locations))
        .route("/api/locations/:loc/rooms", get(crate::routes::get_rooms))
        .route(
            "/api/locations/:loc/rooms/:rid/devices",
            get(crate::routes::get_devices),
        )
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
        .fallback(crate::routes::handler_404)
}
