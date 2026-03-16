mod create_commits;
mod get_commit;
mod get_commit_diff;
mod get_commits;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{Commit, CommitDiff};

use super::RepositoryDiffResponse;

pub use create_commits::CreateCommitsRequest;
pub use get_commit::GetCommitRequest;
pub use get_commit_diff::GetCommitDiffRequest;
pub use get_commits::GetCommitsRequest;

#[derive(Debug, Clone)]
pub struct CommitResponse {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub git_author_name: String,
    pub git_author_email: String,
    pub repo_id: Uuid,
    pub ref_name: String,
    pub sha: String,
    pub parent_sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub diffs: Vec<CommitDiff>,
}

#[derive(Debug, Clone)]
pub struct CommitsResponse {
    pub commits: Vec<CommitResponse>,
    pub has_next: bool,
}

#[derive(Debug, Clone)]
pub struct CommitDiffResponse {
    pub sha: String,
    pub parent_sha: String,
    pub files: Vec<CommitFileDiffResponse>,
}

#[derive(Debug, Clone)]
pub struct CommitFileDiffResponse {
    pub path: String,
    pub left_content: Option<String>,
    pub right_content: Option<String>,
    pub diff: RepositoryDiffResponse,
}

impl From<Commit> for CommitResponse {
    fn from(commit: Commit) -> Self {
        Self {
            id: commit.id,
            author_id: commit.author_id,
            git_author_name: commit.git_author_name,
            git_author_email: commit.git_author_email,
            repo_id: commit.repo_id,
            ref_name: commit.ref_name,
            sha: commit.sha,
            parent_sha: commit.parent_sha,
            message: commit.message,
            created_at: commit.created_at,
            diffs: commit.diffs,
        }
    }
}
