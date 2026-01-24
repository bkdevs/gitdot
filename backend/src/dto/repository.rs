mod create_repository;
mod get_repository_commits;
mod get_repository_file;
mod get_repository_tree;

use chrono::{DateTime, Utc};
use serde::Serialize;

use gitdot_core::dto::RepositoryCommitResponse;

pub use create_repository::{CreateRepositoryServerRequest, CreateRepositoryServerResponse};
pub use get_repository_commits::{GetRepositoryCommitsQuery, GetRepositoryCommitsServerResponse};
pub use get_repository_file::{GetRepositoryFileQuery, GetRepositoryFileServerResponse};
pub use get_repository_tree::{GetRepositoryTreeQuery, GetRepositoryTreeServerResponse};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RepositoryCommitServerResponse {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub date: DateTime<Utc>,
}

impl From<RepositoryCommitResponse> for RepositoryCommitServerResponse {
    fn from(commit: RepositoryCommitResponse) -> Self {
        Self {
            sha: commit.sha,
            message: commit.message,
            author: commit.author,
            date: commit.date,
        }
    }
}

fn default_ref() -> String {
    "HEAD".to_string()
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    30
}
