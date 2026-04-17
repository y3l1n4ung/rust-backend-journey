pub mod axum_basic;
use axum::Router;
use sqlx::SqlitePool;

pub fn create_app(pool: SqlitePool) -> Router {
    axum_basic::router::create_router(pool)
}
