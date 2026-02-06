mod create_commit;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::Commit;

pub use create_commit::CreateCommitRequest;

#[derive(Debug, Clone)]
pub struct CommitResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub repo_id: Uuid,
    pub sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

impl From<Commit> for CommitResponse {
    fn from(commit: Commit) -> Self {
        Self {
            id: commit.id,
            author_id: commit.author_id,
            repo_id: commit.repo_id,
            sha: commit.sha,
            message: commit.message,
            created_at: commit.created_at,
        }
    }
}
