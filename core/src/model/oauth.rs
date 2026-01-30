use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum DeviceAuthorizationStatus {
    Pending,
    Authorized,
    Denied,
    Expired,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DeviceAuthorization {
    pub id: Uuid,
    pub device_code: String,
    pub user_code: String,
    pub user_id: Option<Uuid>,
    pub status: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AccessToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub name: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
