use crate::db::AppState;
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::status_log::DeviceStatusLog;

pub async fn save_device_status_log(
    pool: &PgPool,
    device_id: &str,
    payload: serde_json::Value,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO device_status_logs (id, device_id, payload)
        VALUES ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        device_id,
        payload
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_status_logs_by_device_id(
    pool: &PgPool,
    device_id: &str,
) -> Result<Vec<DeviceStatusLog>> {
    let logs = sqlx::query_as::<_, DeviceStatusLog>(
        r#"
        SELECT id, device_id, payload, created_at
        FROM device_status_logs
        WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .fetch_all(pool)
    .await?;

    Ok(logs)
}