use async_trait::async_trait;

use crate::{
    client::{Git2Client, GitClient, GitHubClient, OctocrabClient},
    dto::{ProcessGithubPushRequest, ProcessGithubPushResponse, SyncedRepositoryInfo},
    error::WebhookError,
    repository::{
        MigrationRepository, MigrationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl,
    },
    util::{git::ZERO_SHA, github::get_github_clone_url},
};

#[async_trait]
pub trait GithubWebhookService: Send + Sync + 'static {
    async fn process_github_push(
        &self,
        request: ProcessGithubPushRequest,
    ) -> Result<ProcessGithubPushResponse, WebhookError>;
}

#[derive(Debug, Clone)]
pub struct GithubWebhookServiceImpl<R, MR, G, GH>
where
    R: RepositoryRepository,
    MR: MigrationRepository,
    G: GitClient,
    GH: GitHubClient,
{
    repo_repo: R,
    migration_repo: MR,
    git_client: G,
    github_client: GH,
}

impl
    GithubWebhookServiceImpl<
        RepositoryRepositoryImpl,
        MigrationRepositoryImpl,
        Git2Client,
        OctocrabClient,
    >
{
    pub fn new(
        repo_repo: RepositoryRepositoryImpl,
        migration_repo: MigrationRepositoryImpl,
        git_client: Git2Client,
        github_client: OctocrabClient,
    ) -> Self {
        Self {
            repo_repo,
            migration_repo,
            git_client,
            github_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<R, MR, G, GH> GithubWebhookService for GithubWebhookServiceImpl<R, MR, G, GH>
where
    R: RepositoryRepository,
    MR: MigrationRepository,
    G: GitClient,
    GH: GitHubClient,
{
    async fn process_github_push(
        &self,
        request: ProcessGithubPushRequest,
    ) -> Result<ProcessGithubPushResponse, WebhookError> {
        let empty_response = ProcessGithubPushResponse {
            synced_repositories: Vec::new(),
        };

        let default_ref = format!("refs/heads/{}", request.repository.default_branch);
        if request.ref_name != default_ref {
            // only sync pushes to the default branch
            return Ok(empty_response);
        }
        if request.after == ZERO_SHA {
            return Ok(empty_response);
        }

        let origin_full_name = format!(
            "{}/{}",
            request.repository.owner.login, request.repository.name
        );
        let migration_repositories = self
            .migration_repo
            .list_by_origin_repository_id(request.repository.id)
            .await?;
        if migration_repositories.is_empty() {
            return Ok(empty_response);
        }

        let token = self
            .github_client
            .get_installation_access_token(request.installation.id as u64)
            .await?;
        let url = get_github_clone_url(&token, &origin_full_name);

        let mut synced_repositories = Vec::new();
        for migration_repository in migration_repositories {
            let Some(dest_id) = migration_repository.destination_repository_id else {
                continue;
            };
            let Some(dest) = self.repo_repo.get_by_id(dest_id).await? else {
                continue;
            };
            if !dest.readonly {
                continue;
            }

            match self
                .git_client
                .fetch_ref(
                    &dest.owner_name,
                    &dest.name,
                    &url,
                    &request.ref_name,
                    &request.after,
                )
                .await
            {
                Ok(()) => synced_repositories.push(SyncedRepositoryInfo {
                    owner_name: dest.owner_name,
                    repo_name: dest.name,
                    head_sha: request.after.clone(),
                }),
                Err(e) => {
                    tracing::error!(
                        ?e,
                        owner = %dest.owner_name,
                        repo = %dest.name,
                        "failed to fetch ref into readonly mirror",
                    );
                }
            }
        }

        Ok(ProcessGithubPushResponse {
            synced_repositories,
        })
    }
}
