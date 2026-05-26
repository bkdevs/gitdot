use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::resource::{
    build::BuildResource, question::QuestionResource, review::ReviewResource, user::UserResource,
};

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryResource {
    pub id: Uuid,
    pub name: String,
    pub owner: String,
    pub visibility: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub stars: u32,
    pub user_star: bool,
    pub readonly: bool,
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
    pub commit_sha: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryFolderResource {
    pub commit_sha: String,
    pub path: String,
    pub entries: Vec<RepositoryPathResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBlobsResource {
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
    pub owner_name: String,
    pub repo_name: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub git_name: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_content: Option<String>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBlobDiffsResource {
    pub diffs: HashMap<String, RepositoryDiffFileResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitDiffResource {
    pub sha: String,
    pub parent_sha: String,
    pub files: Vec<RepositoryDiffFileResource>,
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
pub struct RepositoryBuildsResource {
    pub builds: Vec<BuildResource>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub builds: Option<RepositoryBuildsResource>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RepositoryActivityEventResource {
    Starred {
        user: UserResource,
        at: DateTime<Utc>,
    },
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryCommitFilterResource {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
