use async_trait::async_trait;

use crate::{
    error::GitHubError,
    model::{GitHubInstallation, GitHubRepository},
};

#[async_trait]
pub trait GitHubClient: Send + Sync + Clone + 'static {
    async fn get_installation(
        &self,
        installation_id: u64,
    ) -> Result<GitHubInstallation, GitHubError>;

    async fn get_installation_access_token(
        &self,
        installation_id: u64,
    ) -> Result<String, GitHubError>;

    async fn list_installation_repositories(
        &self,
        access_token: &str,
    ) -> Result<Vec<GitHubRepository>, GitHubError>;
}

pub struct GitHubClientImpl {}
