use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

// TODO: return list of locations
pub async fn get_locations() -> impl IntoResponse {
    Json(vec!["Location 1", "Location 2"])
}

// TODO: return list of rooms
pub async fn get_rooms(Path(location_name): Path<String>) -> impl IntoResponse {
    match location_name.as_str() {
        "House" => Json(vec!["Room 1", "Room 2"]).into_response(),
        _ => (StatusCode::NOT_FOUND).into_response(),
    }
}

// TODO: return list of devices in room
pub async fn get_devices(
    Path((location_name, _room_name)): Path<(String, String)>,
) -> impl IntoResponse {
    match location_name.as_str() {
        "House" => Json(vec!["Device 1", "Device 2"]).into_response(),
        _ => (StatusCode::NOT_FOUND).into_response(),
    }
}
