use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Commit {
    pub id: Uuid,
    pub author_id: Uuid,
    pub repo_id: Uuid,
    pub ref_name: String,
    pub sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
