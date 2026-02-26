use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Build {
    pub id: Uuid,
    pub number: i32,
    pub repository_id: Uuid,
    pub trigger: String,
    pub commit_sha: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
