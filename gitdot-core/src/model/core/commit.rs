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

    pub owner_name: String,
    pub repo_name: String,

    pub ref_name: String,
    pub sha: String,
    pub parent_sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,

    pub review_number: Option<i32>,
    pub diff_position: Option<i32>,

    #[sqlx(json)]
    pub diffs: Vec<CommitDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitDiff {
    pub path: String,
    pub lines_added: i32,
    pub lines_removed: i32,
}
