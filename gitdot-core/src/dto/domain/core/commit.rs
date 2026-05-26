mod create_commits;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::RepositoryDiffFileResponse;
use crate::model::{Commit, CommitDiff};

pub use create_commits::CreateCommitsRequest;

#[derive(Debug, Clone)]
pub struct CommitResponse {
    pub id: Uuid,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub git_author_name: String,
    pub git_author_email: String,
    pub repo_id: Uuid,
    pub owner_name: String,
    pub repo_name: String,

    pub ref_name: String,
    pub sha: String,
    pub parent_sha: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub review_number: Option<i32>,
    pub diff_position: Option<i32>,
    pub diffs: Vec<CommitDiff>,
}

#[derive(Debug, Clone)]
pub struct CommitDiffResponse {
    pub sha: String,
    pub parent_sha: String,
    pub files: Vec<RepositoryDiffFileResponse>,
}

impl From<Commit> for CommitResponse {
    fn from(commit: Commit) -> Self {
        Self {
            id: commit.id,
            author_id: commit.author_id,
            author_name: commit.author_name,
            git_author_name: commit.git_author_name,
            git_author_email: commit.git_author_email,
            repo_id: commit.repo_id,
            owner_name: commit.repository.owner_name,
            repo_name: commit.repository.name,
            ref_name: commit.ref_name,
            sha: commit.sha,
            parent_sha: commit.parent_sha,
            message: commit.message,
            created_at: commit.created_at,
            review_number: commit.review_number,
            diff_position: commit.diff_position,
            diffs: commit.diffs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserCommitResponse {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub redacted: bool,

    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub git_author_name: Option<String>,
    pub git_author_email: Option<String>,
    pub repo_id: Option<Uuid>,
    pub owner_name: Option<String>,
    pub repo_name: Option<String>,
    pub ref_name: Option<String>,
    pub sha: Option<String>,
    pub parent_sha: Option<String>,
    pub message: Option<String>,
    pub review_number: Option<i32>,
    pub diff_position: Option<i32>,
    pub diffs: Vec<CommitDiff>,
}

impl UserCommitResponse {
    pub fn visible(commit: Commit) -> Self {
        Self {
            id: commit.id,
            created_at: commit.created_at,
            redacted: false,
            author_id: commit.author_id,
            author_name: commit.author_name,
            git_author_name: Some(commit.git_author_name),
            git_author_email: Some(commit.git_author_email),
            repo_id: Some(commit.repo_id),
            owner_name: Some(commit.repository.owner_name),
            repo_name: Some(commit.repository.name),
            ref_name: Some(commit.ref_name),
            sha: Some(commit.sha),
            parent_sha: Some(commit.parent_sha),
            message: Some(commit.message),
            review_number: commit.review_number,
            diff_position: commit.diff_position,
            diffs: commit.diffs,
        }
    }

    pub fn redacted(commit: &Commit) -> Self {
        Self {
            id: commit.id,
            created_at: commit.created_at,
            redacted: true,
            author_id: None,
            author_name: None,
            git_author_name: None,
            git_author_email: None,
            repo_id: None,
            owner_name: None,
            repo_name: None,
            ref_name: None,
            sha: None,
            parent_sha: None,
            message: None,
            review_number: None,
            diff_position: None,
            diffs: Vec::new(),
        }
    }
}
