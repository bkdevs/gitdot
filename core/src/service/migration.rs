use async_trait::async_trait;

use crate::{
    client::GitHubClient,
    dto::{
        CreateGitHubInstallationRequest, GitHubInstallationResponse,
        ListGitHubInstallationRepositoriesResponse, ListGitHubInstallationsRequest,
        ListGitHubInstallationsResponse,
    },
    error::MigrationError,
    model::GitHubInstallationType,
    repository::{GitHubRepository, GitHubRepositoryImpl},
};

#[async_trait]
pub trait MigrationService: Send + Sync + 'static {
    async fn create_github_installation(
        &self,
        request: CreateGitHubInstallationRequest,
    ) -> Result<GitHubInstallationResponse, MigrationError>;

    async fn list_github_installations(
        &self,
        request: ListGitHubInstallationsRequest,
    ) -> Result<ListGitHubInstallationsResponse, MigrationError>;

    async fn list_github_installation_repositories(
        &self,
        installation_id: i64,
    ) -> Result<ListGitHubInstallationRepositoriesResponse, MigrationError>;
}

#[derive(Debug, Clone)]
pub struct MigrationServiceImpl<R, C>
where
    R: GitHubRepository,
    C: GitHubClient,
{
    github_repo: R,
    github_client: C,
}

impl<C: GitHubClient> MigrationServiceImpl<GitHubRepositoryImpl, C> {
    pub fn new(github_repo: GitHubRepositoryImpl, github_client: C) -> Self {
        Self {
            github_repo,
            github_client,
        }
    }
}

#[async_trait]
impl<R, C> MigrationService for MigrationServiceImpl<R, C>
where
    R: GitHubRepository,
    C: GitHubClient,
{
    async fn create_github_installation(
        &self,
        request: CreateGitHubInstallationRequest,
    ) -> Result<GitHubInstallationResponse, MigrationError> {
        let installation = self
            .github_client
            .get_installation(request.installation_id as u64)
            .await?;

        let installation_type = match installation.target_type.as_deref() {
            Some("Organization") => GitHubInstallationType::Organization,
            _ => GitHubInstallationType::User,
        };

        let installation = self
            .github_repo
            .create(
                request.installation_id,
                request.owner_id,
                installation_type,
                &installation.account.login,
            )
            .await?;

        Ok(installation.into())
    }

    async fn list_github_installations(
        &self,
        request: ListGitHubInstallationsRequest,
    ) -> Result<ListGitHubInstallationsResponse, MigrationError> {
        let installations = self.github_repo.list_by_owner(request.owner_id).await?;
        Ok(installations.into_iter().map(Into::into).collect())
    }

    async fn list_github_installation_repositories(
        &self,
        installation_id: i64,
    ) -> Result<ListGitHubInstallationRepositoriesResponse, MigrationError> {
        let repos = self
            .github_client
            .list_installation_repositories(installation_id as u64)
            .await?;

        Ok(repos.repositories.into_iter().map(Into::into).collect())
    }
}
