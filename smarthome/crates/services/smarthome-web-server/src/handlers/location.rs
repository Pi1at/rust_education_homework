use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::LocationData;

pub async fn list(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    (sqlx::query_as!(LocationData, "SELECT id,name FROM location")
        .fetch_all(&pool)
        .await)
        .map_or_else(
            |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            |locations| (StatusCode::OK, Json(locations)).into_response(),
        )
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    Json(loc): Json<LocationData>,
) -> impl IntoResponse {
    let loc_id = Uuid::new_v4();
    (sqlx::query!(
        r#"
        INSERT INTO location (id, name)
        VALUES ($1, $2)
        "#,
        loc_id,
        loc.name
    )
    .execute(&pool)
    .await)
        .map_or_else(
            |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            |_| (StatusCode::OK, Json(loc_id)).into_response(),
        )
}
pub async fn read(
    Extension(pool): Extension<PgPool>,
    Path(loc_id): Path<Uuid>,
) -> impl IntoResponse {
    (sqlx::query_as!(
        LocationData,
        r#"
        SELECT id, name
        FROM location
        WHERE id = $1
        "#,
        loc_id
    )
    .fetch_one(&pool)
    .await)
        .map_or_else(
            |e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            |location| (StatusCode::OK, Json(location)).into_response(),
        )
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(loc_id): Path<Uuid>,
    Json(loc): Json<LocationData>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        UPDATE location SET name = $2
        WHERE id = $1
        "#,
        loc_id,
        loc.name
    )
    .execute(&pool)
    .await
    .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| StatusCode::OK)
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Path(loc_id): Path<Uuid>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"
        DELETE FROM location
        WHERE id = $1
        "#,
        loc_id
    )
    .execute(&pool)
    .await
    .map_or(StatusCode::INTERNAL_SERVER_ERROR, |_| StatusCode::OK)
}
