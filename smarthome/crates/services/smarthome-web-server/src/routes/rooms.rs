use axum::{routing::get, Router};

use crate::handlers::{devices, room};

pub fn room_route() -> Router {
    Router::new()
        .route(
            "/:room_id",
            get(room::read).post(room::update).delete(room::delete),
        )
        .route(
            "/:room_id/devices",
            get(devices::list).post(devices::create),
        )
}
