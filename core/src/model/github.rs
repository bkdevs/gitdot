use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct GitHubInstallation {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
