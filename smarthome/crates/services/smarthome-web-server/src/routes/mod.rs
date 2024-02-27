mod health_check;
mod location_schema;

pub use health_check::*;
pub use location_schema::*;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Smarthome web server</h1>")
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
