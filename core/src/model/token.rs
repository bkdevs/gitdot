use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct DeviceAuthorization {
    pub id: Uuid,
    pub device_code: String,
    pub user_code: String,
    pub client_id: String,
    pub user_id: Option<Uuid>,
    pub status: DeviceAuthorizationStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "device_authorization_status", rename_all = "lowercase")]
pub enum DeviceAuthorizationStatus {
    Pending,
    Authorized,
    Expired,
}

#[derive(Debug, Clone, FromRow)]
pub struct AccessToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_id: String,
    pub token_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}
