use serde::Deserialize;

use super::{GithubInstallation, GithubPushCommit, GithubPusher, GithubRepository};
use crate::error::{InputError, WebhookError};

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

impl ProcessGithubPushRequest {
    pub fn new(body: &[u8]) -> Result<Self, WebhookError> {
        serde_json::from_slice(body)
            .map_err(|e| InputError::new("github push body", e.to_string()).into())
    }
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
