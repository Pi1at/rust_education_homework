use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::RoomData;

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Path(loc_id): Path<Uuid>,
) -> impl IntoResponse {
    (sqlx::query_as!(
        RoomData,
        r#"
        SELECT id, name
        FROM room
        WHERE loc_id = $1
        "#,
        loc_id
    ))
    .fetch_all(&pool)
    .await
    .map_or_else(
        |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        |rooms| (StatusCode::OK, Json(rooms)).into_response(),
    )
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Path(loc_id): Path<Uuid>,
    Json(room): Json<RoomData>,
) -> impl IntoResponse {
    let room_id = Uuid::new_v4();
    (sqlx::query!(
        r#"
        INSERT INTO room (id, name, loc_id)
        VALUES ($1, $2, $3)
        "#,
        room_id,
        room.name,
        loc_id
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
    Path(room_id): Path<Uuid>,
) -> impl IntoResponse {
    (sqlx::query_as!(
        RoomData,
        r#"
        SELECT id, name
        FROM room
        WHERE id = $1
        "#,
        room_id
    )
    .fetch_one(&pool)
    .await)
        .map_or_else(
            |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            |room| (StatusCode::OK, Json(room)).into_response(),
        )
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(room_id): Path<Uuid>,
    Json(room): Json<RoomData>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        UPDATE room SET name = $2
        WHERE id = $1
        "#,
        room_id,
        room.name
    )
    .execute(&pool)
    .await
    .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| StatusCode::OK)
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Path(room_id): Path<Uuid>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        DELETE FROM room
        WHERE id = $1
        "#,
        room_id
    )
    .execute(&pool)
    .await
    .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| StatusCode::OK)
}
