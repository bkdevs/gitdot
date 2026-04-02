use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AuthCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code_hash: String,

    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub refresh_token_family: Uuid,

    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,

    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}
