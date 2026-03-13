use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Commit {
    pub id: Uuid,
    pub repo_id: Uuid,

    pub author_id: Option<Uuid>,
    pub git_author_name: String,
    pub git_author_email: String,

    pub ref_name: String,
    pub sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,

    #[sqlx(json)]
    pub diffs: Vec<Diff>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Diff {
    pub path: String,
    pub lines_added: i32,
    pub lines_removed: i32,
}
