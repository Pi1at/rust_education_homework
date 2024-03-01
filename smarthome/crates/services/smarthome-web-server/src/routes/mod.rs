use axum::Router;

mod devices;
mod locations;
mod rooms;

pub use devices::device_route;
pub use locations::location_route;
pub use rooms::room_route;

pub fn location_schema() -> Router {
    Router::new()
        .nest("/locations", location_route())
        .nest("/rooms", room_route())
        .nest("/devices", device_route())
}
