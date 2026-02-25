use async_trait::async_trait;

use crate::{
    client::{Git2Client, GitClient, GitHubClient, OctocrabClient},
    dto::{
        CreateGitHubInstallationRequest, GitHubInstallationResponse,
        ListGitHubInstallationRepositoriesResponse, ListGitHubInstallationsRequest,
        ListGitHubInstallationsResponse, MigrateGitHubRepositoriesRequest, MigrationResponse,
    },
    error::MigrationError,
    model::{
        GitHubInstallationType, MigrationOrigin, MigrationRepositoryStatus, MigrationStatus,
        RepositoryOwnerType, RepositoryVisibility,
    },
    repository::{
        GitHubRepository, GitHubRepositoryImpl, MigrationRepository, MigrationRepositoryImpl,
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl,
    },
    util::{
        git::{GitHookType, POST_RECEIVE_SCRIPT},
        github::get_github_clone_url,
    },
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

    async fn migrate_github_repositories(
        &self,
        request: MigrateGitHubRepositoriesRequest,
    ) -> Result<MigrationResponse, MigrationError>;
}

#[derive(Debug, Clone)]
pub struct MigrationServiceImpl<G, GH, RR, MR, OR, GHR>
where
    G: GitClient,
    GH: GitHubClient,
    RR: RepositoryRepository,
    MR: MigrationRepository,
    OR: OrganizationRepository,
    GHR: GitHubRepository,
{
    git_client: G,
    github_client: GH,
    repo_repo: RR,
    migration_repo: MR,
    org_repo: OR,
    github_repo: GHR,
}

impl
    MigrationServiceImpl<
        Git2Client,
        OctocrabClient,
        RepositoryRepositoryImpl,
        MigrationRepositoryImpl,
        OrganizationRepositoryImpl,
        GitHubRepositoryImpl,
    >
{
    pub fn new(
        git_client: Git2Client,
        github_client: OctocrabClient,
        repo_repo: RepositoryRepositoryImpl,
        migration_repo: MigrationRepositoryImpl,
        org_repo: OrganizationRepositoryImpl,
        github_repo: GitHubRepositoryImpl,
    ) -> Self {
        Self {
            git_client,
            github_client,
            repo_repo,
            migration_repo,
            org_repo,
            github_repo,
        }
    }
}

#[async_trait]
impl<G, GH, RR, MR, OR, GHR> MigrationService for MigrationServiceImpl<G, GH, RR, MR, OR, GHR>
where
    G: GitClient,
    GH: GitHubClient,
    RR: RepositoryRepository,
    MR: MigrationRepository,
    OR: OrganizationRepository,
    GHR: GitHubRepository,
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

    async fn migrate_github_repositories(
        &self,
        request: MigrateGitHubRepositoriesRequest,
    ) -> Result<MigrationResponse, MigrationError> {
        let owner_id = match request.owner_type {
            RepositoryOwnerType::User => request.user_id,
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(request.owner_name.as_ref())
                    .await?
                    .ok_or_else(|| MigrationError::OwnerNotFound(request.owner_name.to_string()))?;
                org.id
            }
        };

        let token = self
            .github_client
            .get_installation_access_token(request.installation_id as u64)
            .await?;

        let migration = self
            .migration_repo
            .create(request.user_id, MigrationOrigin::GitHub)
            .await?;
        let mut migration_repositories = Vec::new();
        for full_name in &request.repositories {
            let migration_repository = self
                .migration_repo
                .create_migration_repository(migration.id, full_name)
                .await?;
            migration_repositories.push(migration_repository);
        }
        let response =
            MigrationResponse::from_parts(migration.clone(), migration_repositories.clone());

        self.migration_repo
            .update_status(migration.id, MigrationStatus::Running)
            .await?;

        // migrate repositories in the background

        Ok(response)
    }
}
