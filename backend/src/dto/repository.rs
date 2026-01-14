use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRepositoryRequest {
    #[serde(default = "default_branch")]
    pub default_branch: String,
}

#[derive(Serialize)]
pub struct CreateRepositoryResponse {
    pub owner: String,
    pub name: String,
    pub default_branch: String,
}

#[derive(Deserialize)]
pub struct RepositoryTreeQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default)]
    pub path: String,
}

#[derive(Serialize)]
pub struct RepositoryTree {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub entries: Vec<RepositoryTreeEntry>,
}

#[derive(Serialize)]
pub struct RepositoryTreeEntry {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    pub commit: RepositoryCommit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

#[derive(Deserialize)]
pub struct RepositoryFileQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

#[derive(Serialize, Clone)]
pub struct RepositoryFile {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Deserialize)]
pub struct RepositoryCommitsQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Deserialize)]
pub struct RepositoryFileCommitsQuery {
    pub path: String,
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Serialize)]
pub struct RepositoryCommits {
    pub commits: Vec<RepositoryCommit>,
    pub has_next: bool,
}

#[derive(Serialize, Clone)]
pub struct RepositoryCommit {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Clone)]
pub struct RepositoryFileDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<RepositoryFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<RepositoryFile>,
}

#[derive(Serialize)]
pub struct RepositoryCommitDiffs {
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_sha: Option<String>,
    pub commit: RepositoryCommit,
    pub diffs: Vec<RepositoryFileDiff>,
}

fn default_branch() -> String {
    "main".to_string()
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
