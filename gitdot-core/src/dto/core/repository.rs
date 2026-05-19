mod create_repository;
mod create_repository_commit_filter;
mod delete_repository;
mod delete_repository_commit_filter;
mod get_repository;
mod get_repository_activity;
mod get_repository_blob;
mod get_repository_blob_diffs;
mod get_repository_blobs;
mod get_repository_commit;
mod get_repository_commit_diff;
mod get_repository_file;
mod get_repository_paths;
mod list_repository_commit_filters;
mod list_repository_commits;
mod star_repository;
mod unstar_repository;
mod update_repository_commit_filter;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{CommitFilter, Repository};

pub use create_repository::CreateRepositoryRequest;
pub use create_repository_commit_filter::CreateRepositoryCommitFilterRequest;
pub use delete_repository::DeleteRepositoryRequest;
pub use delete_repository_commit_filter::DeleteRepositoryCommitFilterRequest;
pub use get_repository::GetRepositoryRequest;
pub use get_repository_activity::{GetRepositoryActivityRequest, RepositoryActivityEvent};
pub use get_repository_blob::{
    GetRepositoryBlobRequest, RepositoryBlobResponse, RepositoryFolderResponse,
};
pub use get_repository_blob_diffs::{GetRepositoryBlobDiffsRequest, RepositoryBlobDiffsResponse};
pub use get_repository_blobs::{GetRepositoryBlobsRequest, RepositoryBlobsResponse};
pub use get_repository_commit::GetRepositoryCommitRequest;
pub use get_repository_commit_diff::GetRepositoryCommitDiffRequest;
pub use get_repository_file::RepositoryFileResponse;
pub use get_repository_paths::{
    GetRepositoryPathsRequest, PathType, RepositoryPath, RepositoryPathsResponse,
};
pub use list_repository_commit_filters::ListRepositoryCommitFiltersRequest;
pub use list_repository_commits::ListRepositoryCommitsRequest;
pub use star_repository::StarRepositoryRequest;
pub use unstar_repository::UnstarRepositoryRequest;
pub use update_repository_commit_filter::UpdateRepositoryCommitFilterRequest;

#[derive(Debug, Clone)]
pub struct RepositoryResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    pub description: Option<String>,
    pub stars: u32,
    pub user_star: bool,
    pub readonly: bool,
    pub created_at: DateTime<Utc>,
}

impl RepositoryResponse {
    pub fn with_user_star(mut self, user_star: bool) -> Self {
        self.user_star = user_star;
        self
    }
}

impl From<Repository> for RepositoryResponse {
    fn from(repo: Repository) -> Self {
        Self {
            id: repo.id,
            name: repo.name,
            owner: repo.owner_name,
            visibility: repo.visibility.into(),
            description: repo.description,
            stars: repo.stars as u32,
            user_star: false,
            readonly: repo.readonly,
            created_at: repo.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryCommitsResponse {
    pub commits: Vec<RepositoryCommitResponse>,
}

#[derive(Debug, Clone)]
pub struct RepositoryCommitResponse {
    pub sha: String,
    pub parent_sha: Option<String>,
    pub message: String,
    pub date: DateTime<Utc>,
    pub author: CommitAuthorResponse,
}

#[derive(Debug, Clone)]
pub struct CommitAuthorResponse {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}

impl From<&git2::Commit<'_>> for RepositoryCommitResponse {
    fn from(commit: &git2::Commit) -> Self {
        let git_author = commit.author();
        Self {
            sha: commit.id().to_string(),
            parent_sha: commit.parent(0).ok().map(|p| p.id().to_string()),
            message: commit.message().unwrap_or("").to_string(),
            date: DateTime::from_timestamp(git_author.when().seconds(), 0).unwrap_or_default(),
            author: CommitAuthorResponse {
                id: None,
                name: git_author.name().unwrap_or("Unknown").to_string(),
                email: git_author.email().unwrap_or("").to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryDiffStatResponse {
    pub path: String,
    pub lines_added: u32,
    pub lines_removed: u32,
}

#[derive(Debug, Clone)]
pub struct RepositoryDiffFileResponse {
    pub path: String,
    pub left_content: Option<String>,
    pub right_content: Option<String>,
    pub lines_added: u32,
    pub lines_removed: u32,
}

#[derive(Debug, Clone)]
pub struct RepositoryCommitFilterResponse {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CommitFilter> for RepositoryCommitFilterResponse {
    fn from(f: CommitFilter) -> Self {
        Self {
            id: f.id,
            repository_id: f.repository_id,
            name: f.name,
            authors: f.authors,
            tags: f.tags,
            paths: f.paths,
            created_at: f.created_at,
            updated_at: f.updated_at,
        }
    }
}
