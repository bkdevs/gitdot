mod create_repository;
mod get_repository_commit;
mod get_repository_commits;
mod get_repository_file;
mod get_repository_file_commits;
mod get_repository_preview;
mod get_repository_tree;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::Repository;

pub use create_repository::CreateRepositoryRequest;
pub use get_repository_commit::GetRepositoryCommitRequest;
pub use get_repository_commits::GetRepositoryCommitsRequest;
pub use get_repository_file::{GetRepositoryFileRequest, RepositoryFileResponse};
pub use get_repository_file_commits::GetRepositoryFileCommitsRequest;
pub use get_repository_preview::{
    FilePreview, GetRepositoryPreviewRequest, RepositoryPreviewEntry, RepositoryPreviewResponse,
};
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
pub struct RepositoryCommitsResponse {
    pub commits: Vec<RepositoryCommitResponse>,
    pub has_next: bool,
}

#[derive(Debug, Clone)]
pub struct RepositoryCommitResponse {
    pub sha: String,
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
pub struct RepositoryDiffResponse {
    pub lines_added: u32,
    pub lines_removed: u32,
    pub hunks: Vec<DiffHunk>,
}

pub type DiffHunk = Vec<DiffPair>;

#[derive(Debug, Clone)]
pub struct DiffPair {
    pub lhs: Option<DiffLine>,
    pub rhs: Option<DiffLine>,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_number: u32,
    pub changes: Vec<DiffChange>,
}

#[derive(Debug, Clone)]
pub struct DiffChange {
    pub start: u32,
    pub end: u32,
    pub content: String,
    pub highlight: SyntaxHighlight,
}

#[derive(Debug, Clone)]
pub enum SyntaxHighlight {
    Delimiter,
    Normal,
    String,
    Type,
    Comment,
    Keyword,
    TreeSitterError,
}
