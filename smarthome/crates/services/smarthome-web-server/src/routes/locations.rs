use axum::{routing::get, Router};

use crate::handlers::{location, room};

pub fn location_route() -> Router {
    Router::new()
        .route("/", get(location::list).post(location::create))
        .route(
            "/:loc_id",
            get(location::read)
                .post(location::update)
                .delete(location::delete),
        )
        .route("/:loc_id/rooms", get(room::list).post(room::create))
}
