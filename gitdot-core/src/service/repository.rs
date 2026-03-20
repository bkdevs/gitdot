use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    client::{Git2Client, GitClient},
    dto::{
        CommitAuthorResponse, CreateRepositoryRequest, DeleteRepositoryRequest,
        GetRepositoryBlobRequest, GetRepositoryBlobsRequest, GetRepositoryFileCommitsRequest,
        GetRepositoryPathsRequest, GetRepositorySettingsRequest, RepositoryBlobResponse,
        RepositoryBlobsResponse, RepositoryCommitResponse, RepositoryCommitsResponse,
        RepositoryPathsResponse, RepositoryResponse, RepositorySettingsResponse,
        UpdateRepositorySettingsRequest,
    },
    error::RepositoryError,
    model::{RepositoryOwnerType, RepositorySettings},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
    },
    util::git::{GitHookType, POST_RECEIVE_SCRIPT, PRE_RECEIVE_SCRIPT, PROC_RECEIVE_SCRIPT},
};

#[async_trait]
pub trait RepositoryService: Send + Sync + 'static {
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError>;

    async fn get_repository_blob(
        &self,
        request: GetRepositoryBlobRequest,
    ) -> Result<RepositoryBlobResponse, RepositoryError>;

    async fn get_repository_blobs(
        &self,
        request: GetRepositoryBlobsRequest,
    ) -> Result<RepositoryBlobsResponse, RepositoryError>;

    async fn get_repository_paths(
        &self,
        request: GetRepositoryPathsRequest,
    ) -> Result<RepositoryPathsResponse, RepositoryError>;

    async fn get_repository_file_commits(
        &self,
        request: GetRepositoryFileCommitsRequest,
    ) -> Result<RepositoryCommitsResponse, RepositoryError>;

    async fn get_repository_by_id(&self, id: Uuid) -> Result<RepositoryResponse, RepositoryError>;

    async fn delete_repository(
        &self,
        request: DeleteRepositoryRequest,
    ) -> Result<(), RepositoryError>;

    async fn resolve_ref_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<String, RepositoryError>;

    async fn get_repository_settings(
        &self,
        request: GetRepositorySettingsRequest,
    ) -> Result<RepositorySettingsResponse, RepositoryError>;

    async fn update_repository_settings(
        &self,
        request: UpdateRepositorySettingsRequest,
    ) -> Result<RepositorySettingsResponse, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct RepositoryServiceImpl<G, O, R, U>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    U: UserRepository,
{
    git_client: G,
    org_repo: O,
    repo_repo: R,
    user_repo: U,
}

impl
    RepositoryServiceImpl<
        Git2Client,
        OrganizationRepositoryImpl,
        RepositoryRepositoryImpl,
        UserRepositoryImpl,
    >
{
    pub fn new(
        git_client: Git2Client,
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        user_repo: UserRepositoryImpl,
    ) -> Self {
        Self {
            git_client,
            org_repo,
            repo_repo,
            user_repo,
        }
    }
}

impl<G, O, R, U> RepositoryServiceImpl<G, O, R, U>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    U: UserRepository,
{
    async fn enrich_commits_with_users(
        &self,
        commits: &mut [RepositoryCommitResponse],
    ) -> Result<(), RepositoryError> {
        let emails: Vec<_> = commits
            .iter()
            .map(|c| c.author.email.clone())
            .filter(|e| !e.is_empty())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        if emails.is_empty() {
            return Ok(());
        }

        let users = self.user_repo.get_by_emails(&emails).await?;
        let email_to_user: HashMap<_, _> = users.iter().map(|u| (u.email.as_str(), u)).collect();
        for commit in commits {
            if let Some(user) = email_to_user.get(commit.author.email.as_str()) {
                commit.author = CommitAuthorResponse {
                    id: Some(user.id),
                    name: user.name.clone(),
                    email: commit.author.email.clone(),
                };
            }
        }

        Ok(())
    }
}

#[crate::instrument_all]
#[async_trait]
impl<G, O, R, U> RepositoryService for RepositoryServiceImpl<G, O, R, U>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    U: UserRepository,
{
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<RepositoryResponse, RepositoryError> {
        let repo_name = request.name.to_string();
        if self
            .git_client
            .repo_exists(&request.owner_name, &repo_name)
            .await
        {
            return Err(RepositoryError::Duplicate(repo_name));
        }

        let owner_id = match request.owner_type {
            RepositoryOwnerType::User => request.user_id,
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(&request.owner_name)
                    .await?
                    .ok_or_else(|| {
                        RepositoryError::OwnerNotFound(request.owner_name.to_string())
                    })?;
                org.id
            }
        };

        // Create git repo first
        self.git_client
            .create_repo(&request.owner_name, &repo_name)
            .await?;

        // Install gitdot hooks
        self.git_client
            .install_hook(
                &request.owner_name,
                &repo_name,
                GitHookType::PreReceive,
                PRE_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                &request.owner_name,
                &repo_name,
                GitHookType::PostReceive,
                POST_RECEIVE_SCRIPT,
            )
            .await?;
        self.git_client
            .install_hook(
                &request.owner_name,
                &repo_name,
                GitHookType::ProcReceive,
                PROC_RECEIVE_SCRIPT,
            )
            .await?;

        // Insert into DB, delete git repo on failure
        let repository = match self
            .repo_repo
            .create(
                &repo_name,
                owner_id,
                &request.owner_name,
                &request.owner_type,
                &request.visibility,
            )
            .await
        {
            Ok(repo) => repo,
            Err(e) => {
                let _ = self
                    .git_client
                    .delete_repo(&request.owner_name, &repo_name)
                    .await;
                return Err(e.into());
            }
        };

        Ok(repository.into())
    }

    async fn get_repository_blob(
        &self,
        request: GetRepositoryBlobRequest,
    ) -> Result<RepositoryBlobResponse, RepositoryError> {
        let response = self
            .git_client
            .get_repo_blob(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                &request.path,
            )
            .await?;

        match response {
            RepositoryBlobResponse::File(f) => Ok(RepositoryBlobResponse::File(f)),
            RepositoryBlobResponse::Folder(folder) => Ok(RepositoryBlobResponse::Folder(folder)),
        }
    }

    async fn get_repository_blobs(
        &self,
        request: GetRepositoryBlobsRequest,
    ) -> Result<RepositoryBlobsResponse, RepositoryError> {
        self.git_client
            .get_repo_blobs(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                &request.paths,
            )
            .await
            .map_err(Into::into)
    }

    async fn get_repository_paths(
        &self,
        request: GetRepositoryPathsRequest,
    ) -> Result<RepositoryPathsResponse, RepositoryError> {
        self.git_client
            .get_repo_paths(&request.owner_name, &request.name, &request.ref_name)
            .await
            .map_err(Into::into)
    }

    async fn get_repository_file_commits(
        &self,
        request: GetRepositoryFileCommitsRequest,
    ) -> Result<RepositoryCommitsResponse, RepositoryError> {
        let mut response = self
            .git_client
            .get_repo_file_commits(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                &request.path,
                request.page,
                request.per_page,
            )
            .await?;

        self.enrich_commits_with_users(&mut response.commits)
            .await?;

        Ok(response)
    }

    async fn get_repository_by_id(&self, id: Uuid) -> Result<RepositoryResponse, RepositoryError> {
        let repository = self
            .repo_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(id.to_string()))?;

        Ok(repository.into())
    }

    async fn delete_repository(
        &self,
        request: DeleteRepositoryRequest,
    ) -> Result<(), RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();

        let repository = self
            .repo_repo
            .get(owner, repo)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(format!("{}/{}", owner, repo)))?;

        self.git_client.delete_repo(owner, repo).await?;
        self.repo_repo.delete(repository.id).await?;

        Ok(())
    }

    async fn resolve_ref_sha(
        &self,
        owner: &str,
        repo: &str,
        ref_name: &str,
    ) -> Result<String, RepositoryError> {
        self.git_client
            .resolve_ref_sha(owner, repo, ref_name)
            .await
            .map_err(Into::into)
    }

    async fn get_repository_settings(
        &self,
        request: GetRepositorySettingsRequest,
    ) -> Result<RepositorySettingsResponse, RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();
        let settings = self
            .repo_repo
            .get_settings(owner, repo)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(format!("{}/{}", owner, repo)))?;
        Ok(RepositorySettingsResponse {
            commit_filters: settings.commit_filters,
        })
    }

    async fn update_repository_settings(
        &self,
        request: UpdateRepositorySettingsRequest,
    ) -> Result<RepositorySettingsResponse, RepositoryError> {
        let owner = request.owner.as_ref();
        let repo = request.repo.as_ref();
        let patch = RepositorySettings {
            commit_filters: request.commit_filters,
        };
        let settings = self
            .repo_repo
            .update_settings(owner, repo, patch)
            .await?
            .ok_or_else(|| RepositoryError::NotFound(format!("{}/{}", owner, repo)))?;
        Ok(RepositorySettingsResponse {
            commit_filters: settings.commit_filters,
        })
    }
}
