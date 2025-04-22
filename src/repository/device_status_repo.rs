use crate::db::AppState;
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

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
