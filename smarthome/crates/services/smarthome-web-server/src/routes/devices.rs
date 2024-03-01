use axum::{routing::get, Router};

use crate::handlers::devices;

pub fn device_route() -> Router {
    Router::new().route(
        "/:dev_id",
        get(devices::read)
            .post(devices::update)
            .delete(devices::delete),
    )
}
