use axum::{
    extract::State,
    response::Json,
    http::StatusCode,
};
use crate::repository::device_repo;
use crate::db::AppState;
use serde_json::json;
use crate::models::device::NewDevice;

pub async fn get_devices(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let pool = &state.db;

    match device_repo::get_all_devices(pool).await {
        Ok(devices) => Ok(Json(json!({
            "success": true,
            "data": devices
        }))),
        Err(err) => {
            eprintln!("❌ Error fetching devices: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch devices".to_string()))
        }
    }
}

pub async fn create_device(
    State(state): State<AppState>,
    Json(payload): Json<NewDevice>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let pool = &state.db;

    match device_repo::create_device(pool, payload).await {
        Ok(device) => Ok(Json(json!({
            "success": true,
            "data": device
        }))),
        Err(err) => {
            eprintln!("❌ Error creating device: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create device".to_string()))
        }
    }
}
