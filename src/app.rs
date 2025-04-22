use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;
use std::sync::Arc;
use crate::db::AppState;

use crate::routes::devices::{get_devices, create_device};
use crate::routes::device_status::get_device_status_logs;

pub fn create_app(pool: PgPool) -> Router {
    let state = AppState {
        db: Arc::new(pool),
    };

    Router::new()
        .route("/devices", get(get_devices))
        .route("/devices/:id/status_logs", get(get_device_status_logs))
        .with_state(state) // ✅ đúng kiểu AppState, không cần Arc nữa
}