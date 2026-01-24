mod create_repository;
mod get_repository_commits;
mod get_repository_file;
mod get_repository_tree;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::Repository;

pub use create_repository::CreateRepositoryRequest;
pub use get_repository_commits::{GetRepositoryCommitsRequest, RepositoryCommitsResponse};
pub use get_repository_file::{GetRepositoryFileRequest, RepositoryFileResponse};
pub use get_repository_tree::{
    GetRepositoryTreeRequest, RepositoryTreeEntry, RepositoryTreeResponse,
};

#[derive(Debug, Clone)]
pub struct RepositoryResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
}

impl From<Repository> for RepositoryResponse {
    fn from(repo: Repository) -> Self {
        Self {
            id: repo.id,
            name: repo.name,
            owner: repo.owner_name,
            visibility: repo.visibility.into(),
            created_at: repo.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryCommitResponse {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub date: DateTime<Utc>,
}

impl From<&git2::Commit<'_>> for RepositoryCommitResponse {
    fn from(commit: &git2::Commit) -> Self {
        let author = commit.author();
        Self {
            sha: commit.id().to_string(),
            message: commit.message().unwrap_or("").to_string(),
            author: author.name().unwrap_or("Unknown").to_string(),
            date: DateTime::from_timestamp(author.when().seconds(), 0).unwrap_or_default(),
        }
    }
}
