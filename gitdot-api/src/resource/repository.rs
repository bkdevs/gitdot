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
    pub parent_sha: String,
    pub message: String,
    pub date: DateTime<Utc>,
    pub author: CommitAuthorResource,
    pub diffs: Vec<RepositoryDiffResource>,
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
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryFolderResource {
    pub path: String,
    pub entries: Vec<RepositoryPathResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBlobsResource {
    pub ref_name: String,
    pub commit_sha: String,
    pub blobs: Vec<RepositoryBlobResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RepositoryBlobResource {
    File(RepositoryFileResource),
    Folder(RepositoryFolderResource),
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryDiffResource {
    pub path: String,
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
pub struct RepositoryPathsResource {
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPathResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryPathResource {
    pub path: String,
    pub name: String,
    pub path_type: PathType,
    pub sha: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathType {
    Blob,
    Tree,
    Commit,
    Unknown,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitDiffResource {
    pub sha: String,
    pub parent_sha: String,
    pub files: Vec<CommitFileDiffResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitFileDiffResource {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_content: Option<String>,
    pub diff: RepositoryDiffResource,
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
