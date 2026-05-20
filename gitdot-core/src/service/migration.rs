use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient, GitHubClient, OctocrabClient},
    dto::{
        CreateGitHubInstallationRequest, CreateGitHubInstallationResponse,
        CreateGitHubMigrationRequest, CreateGitHubMigrationResponse, GetGitHubAppInstallUrlRequest,
        GetGitHubAppInstallUrlResponse, GetMigrationRequest, GitHubInstallationResponse,
        ListGitHubInstallationRepositoriesRequest, ListGitHubInstallationRepositoriesResponse,
        ListGitHubInstallationsRequest, ListMigrationsRequest, MigrateGitHubRepositoriesRequest,
        MigrateGitHubRepositoriesResponse, MigratedRepositoryInfo, MigrationResponse, Page,
    },
    error::{ConflictError, GitHubError, InputError, MigrationError, OptionNotFoundExt},
    model::{
        GitHubInstallationType, MigrationOriginService, MigrationRepositoryStatus, MigrationStatus,
        Repository, RepositoryOwnerType, RepositoryVisibility,
    },
    repository::{
        GitHubRepository, GitHubRepositoryImpl, MigrationRepository, MigrationRepositoryImpl,
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl,
    },
    util::{
        cursor,
        git::{GitHookType, POST_RECEIVE_SCRIPT, PRE_RECEIVE_SCRIPT, PROC_RECEIVE_SCRIPT},
        github::get_github_clone_url,
    },
};

#[async_trait]
pub trait MigrationService: Send + Sync + 'static {
    async fn get_migration(
        &self,
        request: GetMigrationRequest,
    ) -> Result<MigrationResponse, MigrationError>;

    async fn list_migrations(
        &self,
        request: ListMigrationsRequest,
    ) -> Result<Page<MigrationResponse>, MigrationError>;

    async fn create_github_installation(
        &self,
        request: CreateGitHubInstallationRequest,
    ) -> Result<CreateGitHubInstallationResponse, MigrationError>;

    async fn get_github_app_install_url(
        &self,
        request: GetGitHubAppInstallUrlRequest,
    ) -> Result<GetGitHubAppInstallUrlResponse, MigrationError>;

    async fn list_github_installations(
        &self,
        request: ListGitHubInstallationsRequest,
    ) -> Result<Page<GitHubInstallationResponse>, MigrationError>;

    async fn list_github_installation_repositories(
        &self,
        request: ListGitHubInstallationRepositoriesRequest,
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
        visibility: &RepositoryVisibility,
        token: &str,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<(Repository, Option<String>, String), MigrationError> {
        let repo_name = full_name
            .split('/')
            .nth(1)
            .ok_or_else(|| InputError::new("repository name", full_name))?;

        if self
            .repo_repo
            .get(owner_name, repo_name, None)
            .await?
            .is_some()
        {
            return Err(
                ConflictError::new("repository", format!("{}/{}", owner_name, repo_name)).into(),
            );
        }

        let clone_url = get_github_clone_url(token, full_name);
        self.git_client
            .mirror_repo(owner_name, repo_name, &clone_url)
            .await?;

        let result = self
            .setup_mirrored_repository(
                owner_name, repo_name, owner_id, owner_type, visibility, readonly, created_at,
            )
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
        visibility: &RepositoryVisibility,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<(Repository, Option<String>, String), MigrationError> {
        self.git_client.empty_hooks(owner_name, repo_name).await?;
        self.git_client
            .install_hook(
                owner_name,
                repo_name,
                GitHookType::PreReceive,
                PRE_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                owner_name,
                repo_name,
                GitHookType::PostReceive,
                POST_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                owner_name,
                repo_name,
                GitHookType::ProcReceive,
                PROC_RECEIVE_SCRIPT,
            )
            .await?;

        let repository = self
            .repo_repo
            .create(
                repo_name, owner_id, owner_type, visibility, None, readonly, created_at,
            )
            .await?;

        let head_sha = self
            .git_client
            .get_repo_commit(owner_name, repo_name, "HEAD")
            .await
            .ok()
            .map(|c| c.sha);

        let default_ref_name = self
            .git_client
            .get_default_ref(owner_name, repo_name)
            .await
            .unwrap_or_else(|_| "refs/heads/main".to_string());

        Ok((repository, head_sha, default_ref_name))
    }
}

#[crate::instrument_all(level = "debug")]
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
    async fn get_migration(
        &self,
        request: GetMigrationRequest,
    ) -> Result<MigrationResponse, MigrationError> {
        let migration = self
            .migration_repo
            .get(request.user_id, request.number)
            .await?
            .or_not_found("migration", request.number)?;

        Ok(migration.into())
    }

    async fn list_migrations(
        &self,
        request: ListMigrationsRequest,
    ) -> Result<Page<MigrationResponse>, MigrationError> {
        let (rows, next) = self
            .migration_repo
            .list(request.user_id, request.cursor, request.limit as i64)
            .await?;

        Ok(Page {
            data: rows.into_iter().map(MigrationResponse::from).collect(),
            next_cursor: next.as_ref().map(cursor::encode),
        })
    }

    async fn create_github_installation(
        &self,
        request: CreateGitHubInstallationRequest,
    ) -> Result<CreateGitHubInstallationResponse, MigrationError> {
        let payload = self.github_client.verify_install_state(&request.state)?;
        if payload.user_id != request.owner_id {
            return Err(GitHubError::InvalidState.into());
        }

        let access_token = self.github_client.exchange_code(&request.code).await?;
        let github_user_name = self.github_client.get_user_name(&access_token).await?;

        let installation = self
            .github_client
            .get_installation(request.installation_id as u64)
            .await?;
        let installation_type = match installation.target_type.as_deref() {
            Some("Organization") => GitHubInstallationType::Organization,
            _ => GitHubInstallationType::User,
        };

        match installation_type {
            GitHubInstallationType::User => {
                if github_user_name != installation.account.login {
                    return Err(GitHubError::Unauthorized.into());
                }
            }
            GitHubInstallationType::Organization => {
                let role = self
                    .github_client
                    .get_user_membership(
                        &installation.account.login,
                        &github_user_name,
                        &access_token,
                    )
                    .await?;
                if role != "admin" {
                    return Err(GitHubError::Unauthorized.into());
                }
            }
        }

        let installation = self
            .github_repo
            .create(
                request.installation_id,
                request.owner_id,
                installation_type,
                &installation.account.login,
            )
            .await?;

        Ok(CreateGitHubInstallationResponse {
            installation: installation.into(),
            action: payload.action,
        })
    }

    async fn get_github_app_install_url(
        &self,
        request: GetGitHubAppInstallUrlRequest,
    ) -> Result<GetGitHubAppInstallUrlResponse, MigrationError> {
        let install_url = self
            .github_client
            .get_github_app_install_url(request.owner_id, request.action)?;
        Ok(GetGitHubAppInstallUrlResponse { install_url })
    }

    async fn list_github_installations(
        &self,
        request: ListGitHubInstallationsRequest,
    ) -> Result<Page<GitHubInstallationResponse>, MigrationError> {
        let (installations, next_cursor) = self
            .github_repo
            .list_by_owner(request.owner_id, request.cursor, request.limit as i64)
            .await?;
        Ok(Page {
            data: installations.into_iter().map(Into::into).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn list_github_installation_repositories(
        &self,
        request: ListGitHubInstallationRepositoriesRequest,
    ) -> Result<ListGitHubInstallationRepositoriesResponse, MigrationError> {
        // validate that the installation is owned by the user
        self.github_repo
            .get(request.owner_id, request.installation_id)
            .await?
            .or_not_found("github installation", request.installation_id)?;

        let repos = self
            .github_client
            .list_installation_repositories(request.installation_id as u64)
            .await?;

        Ok(repos.repositories.into_iter().map(Into::into).collect())
    }

    async fn create_github_migration(
        &self,
        request: CreateGitHubMigrationRequest,
    ) -> Result<CreateGitHubMigrationResponse, MigrationError> {
        let owner_id = match request.destination_type {
            RepositoryOwnerType::User => request.author_id,
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(request.destination.as_ref())
                    .await?
                    .or_not_found("owner", request.destination.as_ref())?;
                org.id
            }
        };

        let github_repos = self
            .github_client
            .list_installation_repositories(request.installation_id as u64)
            .await?;

        let migration = self
            .migration_repo
            .create(
                request.author_id,
                MigrationOriginService::GitHub,
                &request.origin,
                &request.origin_type,
                request.destination.as_ref(),
                &request.destination_type,
            )
            .await?;

        let mut repositories = Vec::new();
        for repo in &request.repositories {
            let name = repo.name.as_ref();
            let origin_full_name = format!("{}/{}", request.origin, name);
            let destination_full_name = format!("{}/{}", request.destination.as_ref(), name);
            let github_repo = github_repos
                .repositories
                .iter()
                .find(|r| r.name == name)
                .ok_or_else(|| InputError::new("repository", &origin_full_name))?;
            let visibility = if github_repo.private.unwrap_or(false) {
                RepositoryVisibility::Private
            } else {
                RepositoryVisibility::Public
            };
            let origin_created_at = github_repo.created_at;
            let migration_repository = self
                .migration_repo
                .create_migration_repository(
                    migration.id,
                    &origin_full_name,
                    repo.id,
                    origin_created_at,
                    &destination_full_name,
                    &visibility,
                )
                .await?;
            repositories.push(migration_repository);
        }

        let mut migration = migration;
        migration.repositories = Some(repositories);

        Ok(CreateGitHubMigrationResponse {
            migration,
            owner_id,
            owner_name: request.destination,
            owner_type: request.destination_type,
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

        let readonly = request.readonly;
        let mut handles = Vec::new();
        for migration_repo_entry in request.migration_repositories {
            let service = self.clone();
            let token = token.clone();
            let owner_name = request.owner_name.to_string();
            let owner_id = request.owner_id;
            let owner_type = request.owner_type.clone();
            let migration_repo_id = migration_repo_entry.id;
            let full_name = migration_repo_entry.origin_full_name.clone();
            let visibility = migration_repo_entry.visibility.clone();
            let created_at = migration_repo_entry.origin_created_at;

            let handle = tokio::spawn(async move {
                let _ = service
                    .migration_repo
                    .update_migration_repository_status(
                        migration_repo_id,
                        MigrationRepositoryStatus::Running,
                        None,
                    )
                    .await;

                match service
                    .migrate_single_repository(
                        &owner_name,
                        owner_id,
                        &owner_type,
                        &full_name,
                        &visibility,
                        &token,
                        readonly,
                        created_at,
                    )
                    .await
                {
                    Ok((repository, head_sha, default_ref_name)) => {
                        let _ = service
                            .migration_repo
                            .update_migration_repository_status(
                                migration_repo_id,
                                MigrationRepositoryStatus::Completed,
                                None,
                            )
                            .await;
                        let _ = service
                            .migration_repo
                            .set_destination_repository_id(migration_repo_id, repository.id)
                            .await;

                        Some(MigratedRepositoryInfo {
                            owner_name: owner_name.clone(),
                            repo_name: repository.name,
                            default_ref_name,
                            head_sha,
                        })
                    }
                    Err(e) => {
                        let _ = service
                            .migration_repo
                            .update_migration_repository_status(
                                migration_repo_id,
                                MigrationRepositoryStatus::Failed,
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
