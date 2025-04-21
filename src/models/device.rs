use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: Uuid,
    pub device_id: String,
    pub device_type: Option<String>,
    pub name: Option<String>,
    pub firmware_version: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub ip: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewDevice {
    pub device_id: String,
    pub device_type: Option<String>,
    pub name: Option<String>,
    pub firmware_version: Option<String>,
    pub ip: Option<String>,
}

