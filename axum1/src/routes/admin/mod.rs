mod middleware;

use axum::{http::StatusCode, middleware::from_extractor, routing::get, Json, Router};

use crate::{
    error::ApiError,
    extractors::{DatabaseConnection, RedisConnection},
};

use self::middleware::AdminUser;

#[must_use]
pub fn admin_router() -> Router {
    Router::new()
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/pg", get(pg_health))
        .route("/redis", get(redis_health))
        .route_layer(from_extractor::<AdminUser>())
}

async fn pg_health(DatabaseConnection(mut conn): DatabaseConnection) -> Result<(), ApiError> {
    let _ = sqlx::query_scalar!("SELECT 1 + 1")
        .fetch_one(&mut conn)
        .await?;
    Ok(())
}

async fn redis_health(RedisConnection(conn): RedisConnection) -> Result<Json<usize>, ApiError> {
    Ok(Json(conn.count().await?))
}
