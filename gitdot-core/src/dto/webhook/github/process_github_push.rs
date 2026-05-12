use serde::Deserialize;

use super::{GithubInstallation, GithubPushCommit, GithubPusher, GithubRepository};

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessGithubPushRequest {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub before: String,
    pub after: String,
    pub repository: GithubRepository,
    pub pusher: GithubPusher,
    pub installation: GithubInstallation,
    pub commits: Vec<GithubPushCommit>,
}

#[derive(Debug, Clone)]
pub struct ProcessGithubPushResponse {
    pub synced_repositories: Vec<SyncedRepositoryInfo>,
}

#[derive(Debug, Clone)]
pub struct SyncedRepositoryInfo {
    pub owner_name: String,
    pub repo_name: String,
    pub head_sha: String,
}
