use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryResource {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitsResource {
    pub commits: Vec<RepositoryCommitResource>,
    pub has_next: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitResource {
    pub sha: String,
    pub message: String,
    pub date: DateTime<Utc>,
    pub author: CommitAuthorResource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAuthorResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryFileResource {
    pub name: String,
    pub owner: String,
    pub ref_name: String,
    pub path: String,
    pub commit_sha: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryPreviewResource {
    pub name: String,
    pub owner: String,
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPreviewEntryResource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryPreviewEntryResource {
    pub path: String,
    pub name: String,
    pub sha: String,
    pub preview: Option<FilePreviewResource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilePreviewResource {
    pub content: String,
    pub total_lines: u32,
    pub preview_lines: u32,
    pub truncated: bool,
    pub encoding: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTreeResource {
    pub name: String,
    pub owner: String,
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryTreeEntryResource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTreeEntryResource {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    pub commit: RepositoryCommitResource,
}
