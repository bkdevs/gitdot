use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient, GitHubClient, OctocrabClient},
    dto::{
        CreateGitHubInstallationRequest, CreateGitHubMigrationRequest,
        CreateGitHubMigrationResponse, GitHubInstallationResponse,
        ListGitHubInstallationRepositoriesResponse, ListGitHubInstallationsRequest,
        ListGitHubInstallationsResponse, MigrateGitHubRepositoriesRequest,
        MigrateGitHubRepositoriesResponse, MigratedRepositoryInfo,
    },
    error::MigrationError,
    model::{
        GitHubInstallationType, MigrationOrigin, MigrationRepositoryStatus, MigrationStatus,
        Repository, RepositoryOwnerType, RepositoryVisibility,
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

    async fn create_github_migration(
        &self,
        request: CreateGitHubMigrationRequest,
    ) -> Result<CreateGitHubMigrationResponse, MigrationError>;

    async fn migrate_github_repositories(
        &self,
        request: MigrateGitHubRepositoriesRequest,
    ) -> Result<MigrateGitHubRepositoriesResponse, MigrationError>;
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

impl<G, GH, RR, MR, OR, GHR> MigrationServiceImpl<G, GH, RR, MR, OR, GHR>
where
    G: GitClient,
    GH: GitHubClient,
    RR: RepositoryRepository,
    MR: MigrationRepository,
    OR: OrganizationRepository,
    GHR: GitHubRepository,
{
    async fn migrate_single_repository(
        &self,
        owner_name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
        full_name: &str,
        token: &str,
    ) -> Result<(Repository, Option<String>), MigrationError> {
        let repo_name = full_name
            .split('/')
            .nth(1)
            .ok_or_else(|| MigrationError::InvalidRepositoryName(full_name.to_string()))?;

        let clone_url = get_github_clone_url(token, full_name);
        self.git_client
            .mirror_repo(owner_name, repo_name, &clone_url)
            .await?;

        let result = self
            .setup_mirrored_repository(owner_name, repo_name, owner_id, owner_type)
            .await;
        if result.is_err() {
            let _ = self.git_client.delete_repo(owner_name, repo_name).await;
        }

        result
    }

    async fn setup_mirrored_repository(
        &self,
        owner_name: &str,
        repo_name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
    ) -> Result<(Repository, Option<String>), MigrationError> {
        self.git_client.empty_hooks(owner_name, repo_name).await?;
        self.git_client
            .install_hook(
                owner_name,
                repo_name,
                GitHookType::PostReceive,
                POST_RECEIVE_SCRIPT,
            )
            .await?;

        let repository = self
            .repo_repo
            .create(
                repo_name,
                owner_id,
                owner_name,
                owner_type,
                &RepositoryVisibility::Public,
            )
            .await?;

        let head_sha = self
            .git_client
            .get_repo_commit(owner_name, repo_name, "HEAD")
            .await
            .ok()
            .map(|c| c.sha);

        Ok((repository, head_sha))
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

    async fn create_github_migration(
        &self,
        request: CreateGitHubMigrationRequest,
    ) -> Result<CreateGitHubMigrationResponse, MigrationError> {
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

        Ok(CreateGitHubMigrationResponse {
            migration,
            migration_repositories,
            owner_id,
            owner_name: request.owner_name,
            owner_type: request.owner_type,
        })
    }

    async fn migrate_github_repositories(
        &self,
        request: MigrateGitHubRepositoriesRequest,
    ) -> Result<MigrateGitHubRepositoriesResponse, MigrationError> {
        let token = self
            .github_client
            .get_installation_access_token(request.installation_id as u64)
            .await?;

        self.migration_repo
            .update_status(request.migration_id, MigrationStatus::Running)
            .await?;

        let mut handles = Vec::new();
        for migration_repo_entry in request.migration_repositories {
            let service = self.clone();
            let token = token.clone();
            let owner_name = request.owner_name.to_string();
            let owner_id = request.owner_id;
            let owner_type = request.owner_type.clone();
            let migration_repo_id = migration_repo_entry.id;
            let full_name = migration_repo_entry.full_name.clone();

            let handle = tokio::spawn(async move {
                let _ = service
                    .migration_repo
                    .update_migration_repository_status(
                        migration_repo_id,
                        MigrationRepositoryStatus::Running,
                        None,
                        None,
                    )
                    .await;

                match service
                    .migrate_single_repository(
                        &owner_name,
                        owner_id,
                        &owner_type,
                        &full_name,
                        &token,
                    )
                    .await
                {
                    Ok((repository, head_sha)) => {
                        let _ = service
                            .migration_repo
                            .update_migration_repository_status(
                                migration_repo_id,
                                MigrationRepositoryStatus::Completed,
                                Some(repository.id),
                                None,
                            )
                            .await;

                        Some(MigratedRepositoryInfo {
                            owner_name: owner_name.clone(),
                            repo_name: repository.name,
                            head_sha,
                        })
                    }
                    Err(e) => {
                        let _ = service
                            .migration_repo
                            .update_migration_repository_status(
                                migration_repo_id,
                                MigrationRepositoryStatus::Failed,
                                None,
                                Some(&e.to_string()),
                            )
                            .await;
                        None
                    }
                }
            });

            handles.push(handle);
        }

        let mut migrated = Vec::new();
        let mut all_succeeded = true;
        for handle in handles {
            match handle.await {
                Ok(Some(info)) => migrated.push(info),
                Ok(None) => all_succeeded = false,
                Err(_) => all_succeeded = false,
            }
        }

        let final_status = if all_succeeded {
            MigrationStatus::Completed
        } else {
            MigrationStatus::Failed
        };

        self.migration_repo
            .update_status(request.migration_id, final_status)
            .await?;

        Ok(MigrateGitHubRepositoriesResponse {
            migrated_repositories: migrated,
        })
    }
}
