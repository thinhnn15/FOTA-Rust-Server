use axum::{extract::{Path, State}, Json};
use std::sync::Arc;
use crate::db::AppState;
use crate::repository::device_status_repo;
use crate::models::status_log::DeviceStatusLog;

pub async fn get_device_status_logs(
    Path(device_id): Path<String>,
    State(state): State<AppState>,
) -> Json<Vec<DeviceStatusLog>> {
    match device_status_repo::get_status_logs_by_device_id(&state.db, &device_id).await {
        Ok(logs) => Json(logs),
        Err(_) => Json(vec![]),
    }
}
