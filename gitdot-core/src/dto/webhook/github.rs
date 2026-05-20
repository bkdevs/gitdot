mod process_github_installation;
mod process_github_push;

use serde::Deserialize;

pub use process_github_installation::ProcessGithubInstallationRequest;
pub use process_github_push::{
    ProcessGithubPushRequest, ProcessGithubPushResponse, SyncedRepositoryInfo,
};

#[derive(Debug, Clone, Deserialize)]
pub struct GithubRepository {
    pub id: i64,
    pub name: String,
    pub owner: GithubRepositoryOwner,
    pub default_branch: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubRepositoryOwner {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubPusher {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubInstallation {
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubPushCommit {
    pub id: String,
    pub message: String,
}
