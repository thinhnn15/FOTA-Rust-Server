use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;
use crate::routes::devices::get_devices;
use crate::db::AppState;
use crate::routes::devices::create_device;

pub fn create_app(pool: PgPool) -> Router {
    let state = AppState { db: std::sync::Arc::new(pool) };

    Router::new()
        .route(
            "/devices",
            get(get_devices).post(create_device)
        )
        .with_state(state)
}