use crate::models::device::Device;
use sqlx::PgPool;
use anyhow::Result;
use crate::models::device::NewDevice;
use uuid::Uuid;


pub async fn get_all_devices(pool: &PgPool) -> Result<Vec<Device>> {
    let devices = sqlx::query_as::<_, Device>(
        r#"
        SELECT id, device_id, type as device_type, name, firmware_version, last_seen, ip
        FROM devices
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(devices)
}

pub async fn create_device(pool: &PgPool, new_device: NewDevice) -> Result<Device> {
    let device = sqlx::query_as::<_, Device>(
        r#"
        INSERT INTO devices (id, device_id, type, name, firmware_version, ip, last_seen)
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING id, device_id, type as device_type, name, firmware_version, last_seen, ip
        "#
    )
        .bind(Uuid::new_v4())                      // id
        .bind(&new_device.device_id)
        .bind(&new_device.device_type)
        .bind(&new_device.name)
        .bind(&new_device.firmware_version)
        .bind(&new_device.ip)
        .fetch_one(pool)
        .await?;

    Ok(device)
}

pub async fn device_exists(pool: &PgPool, device_id: &str) -> Result<bool> {
    let record = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM devices WHERE device_id = $1
        )
        "#,
        device_id
    )
    .fetch_one(pool)
    .await?;

    Ok(record.unwrap_or(false))
}
