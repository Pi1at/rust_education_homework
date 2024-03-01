use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::DeviceData;

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Path(room_id): Path<Uuid>,
) -> impl IntoResponse {
    (sqlx::query_as!(
        DeviceData,
        r#"
        SELECT id, name
        FROM device
        WHERE room_id = $1
        "#,
        room_id
    )
    .fetch_all(&pool)
    .await)
        .map_or_else(
            |_| (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
            |devices| (StatusCode::OK, Json(devices)),
        )
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Path(room_id): Path<Uuid>,
    Json(device): Json<DeviceData>,
) -> impl IntoResponse {
    let dev_id = Uuid::new_v4();
    (sqlx::query!(
        r#"
        INSERT INTO device (id, name, room_id)
        VALUES ($1, $2, $3)
        "#,
        dev_id,
        device.name,
        room_id
    )
    .execute(&pool)
    .await)
        .map_or_else(
            |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            |_| (StatusCode::OK, Json(room_id)).into_response(),
        )
}

pub async fn read(
    Extension(pool): Extension<PgPool>,
    Path(device_id): Path<Uuid>,
) -> impl IntoResponse {
    (sqlx::query_as!(
        DeviceData,
        r#"
        SELECT id, name
        FROM device
        WHERE id = $1
        "#,
        device_id
    )
    .fetch_one(&pool)
    .await)
        .map_or_else(
            |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            |device| (StatusCode::OK, Json(device)).into_response(),
        )
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(device_id): Path<Uuid>,
    Json(device): Json<DeviceData>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        UPDATE device SET name = $2
        WHERE id = $1
        "#,
        device_id,
        device.name
    )
    .execute(&pool)
    .await
    .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| StatusCode::OK)
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Path(device_id): Path<Uuid>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        DELETE FROM device
        WHERE id = $1
        "#,
        device_id
    )
    .execute(&pool)
    .await
    .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| StatusCode::OK)
}
