use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::resource::{
    question::QuestionResource, review::ReviewResource, settings::CommitFilterResource,
};

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryResource {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    pub created_at: DateTime<Utc>,
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
pub struct RepositoryCommitsResource {
    pub commits: Vec<RepositoryCommitResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitResource {
    pub sha: String,
    pub parent_sha: String,
    pub message: String,
    pub date: DateTime<Utc>,
    pub author: CommitAuthorResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_position: Option<i32>,
    pub diffs: Vec<RepositoryDiffStatResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAuthorResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryDiffStatResource {
    pub path: String,
    pub lines_added: u32,
    pub lines_removed: u32,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryDiffFileResource {
    pub path: String,
    pub lines_added: u32,
    pub lines_removed: u32,
    pub hunks: Vec<DiffHunkResource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_content: Option<String>,
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

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitDiffResource {
    pub sha: String,
    pub parent_sha: String,
    pub files: Vec<RepositoryDiffFileResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositorySettingsResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_filters: Option<Vec<CommitFilterResource>>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryQuestionsResource {
    pub questions: Vec<QuestionResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryReviewsResource {
    pub reviews: Vec<ReviewResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryResourcesResource {
    pub last_commit: String,
    pub last_updated: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub paths: Option<RepositoryPathsResource>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub commits: Option<RepositoryCommitsResource>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blobs: Option<RepositoryBlobsResource>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub questions: Option<RepositoryQuestionsResource>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviews: Option<RepositoryReviewsResource>,
}
