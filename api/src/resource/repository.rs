use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryResource {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitsResource {
    pub commits: Vec<RepositoryCommitResource>,
    pub has_next: bool,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitResource {
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_sha: Option<String>,
    pub message: String,
    pub date: DateTime<Utc>,
    pub author: CommitAuthorResource,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAuthorResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryFileResource {
    pub ref_name: String,
    pub path: String,
    pub commit_sha: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryPreviewResource {
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPreviewEntryResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryPreviewEntryResource {
    pub path: String,
    pub name: String,
    pub sha: String,
    pub preview: Option<FilePreviewResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilePreviewResource {
    pub content: String,
    pub total_lines: u32,
    pub preview_lines: u32,
    pub truncated: bool,
    pub encoding: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTreeResource {
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryTreeEntryResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTreeEntryResource {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    pub commit: RepositoryCommitResource,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitStatResource {
    pub path: String,
    pub lines_added: u32,
    pub lines_removed: u32,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitDiffResource {
    pub diff: RepositoryDiffResource,
    pub left: Option<RepositoryFileResource>,
    pub right: Option<RepositoryFileResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryDiffResource {
    pub lines_added: u32,
    pub lines_removed: u32,
    pub hunks: Vec<DiffHunkResource>,
}

pub type DiffHunkResource = Vec<DiffPairResource>;

#[derive(ApiResource, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct DiffPairResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lhs: Option<DiffLineResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rhs: Option<DiffLineResource>,
}

#[derive(ApiResource, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct DiffLineResource {
    pub line_number: u32,
    pub changes: Vec<DiffChangeResource>,
}

#[derive(ApiResource, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct DiffChangeResource {
    pub start: u32,
    pub end: u32,
    pub content: String,
    pub highlight: SyntaxHighlight,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryPermissionResource {
    pub permission: String,
}

#[derive(ApiResource, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyntaxHighlight {
    Delimiter,
    Normal,
    String,
    Type,
    Comment,
    Keyword,
    TreeSitterError,
}
