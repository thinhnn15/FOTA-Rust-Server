use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeviceStatusLog {
    pub id: Uuid,
    pub device_id: String,
    pub payload: Value,
    pub created_at: DateTime<Utc>,
}
