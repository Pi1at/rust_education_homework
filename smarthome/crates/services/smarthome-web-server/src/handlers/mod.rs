pub mod devices;
mod health_check;
pub mod location;
pub mod room;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub use health_check::*;

pub async fn root() -> Html<&'static str> {
    Html("<h1>Smarthome web server</h1>")
}

pub async fn not_found_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
