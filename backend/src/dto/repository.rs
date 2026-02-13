mod create_repository;
mod get_repository_commits;
mod get_repository_file;
mod get_repository_file_commits;
mod get_repository_preview;
mod get_repository_tree;

use api::resource::RepositoryResource;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use gitdot_core::dto::{CommitAuthorResponse, RepositoryCommitResponse, RepositoryResponse};

pub use create_repository::CreateRepositoryServerRequest;
pub use get_repository_commits::{GetRepositoryCommitsQuery, GetRepositoryCommitsServerResponse};
pub use get_repository_file::{GetRepositoryFileQuery, GetRepositoryFileServerResponse};
pub use get_repository_file_commits::GetRepositoryFileCommitsQuery;
pub use get_repository_preview::{GetRepositoryPreviewQuery, GetRepositoryPreviewServerResponse};
pub use get_repository_tree::{GetRepositoryTreeQuery, GetRepositoryTreeServerResponse};

use super::IntoApi;

impl IntoApi for RepositoryResponse {
    type ApiType = RepositoryResource;
    fn into_api(self) -> Self::ApiType {
        RepositoryResource {
            id: self.id,
            name: self.name,
            owner: self.owner,
            visibility: self.visibility,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RepositoryServerResponse {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
}

impl From<RepositoryResponse> for RepositoryServerResponse {
    fn from(response: RepositoryResponse) -> Self {
        Self {
            id: response.id,
            name: response.name,
            owner: response.owner,
            visibility: response.visibility,
            created_at: response.created_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RepositoryCommitServerResponse {
    pub sha: String,
    pub message: String,
    pub date: DateTime<Utc>,
    pub author: CommitAuthorServerResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommitAuthorServerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}

impl From<CommitAuthorResponse> for CommitAuthorServerResponse {
    fn from(author: CommitAuthorResponse) -> Self {
        Self {
            id: author.id,
            name: author.name,
            email: author.email,
        }
    }
}

impl From<RepositoryCommitResponse> for RepositoryCommitServerResponse {
    fn from(commit: RepositoryCommitResponse) -> Self {
        Self {
            sha: commit.sha,
            message: commit.message,
            date: commit.date,
            author: commit.author.into(),
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
