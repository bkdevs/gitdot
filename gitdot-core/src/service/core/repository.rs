use std::collections::HashMap;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    client::{DiffClient, DifftClient, Git2Client, GitClient},
    dto::{
        CreateRepositoryRequest, DeleteRepositoryRequest, GetRepositoryBlobDiffsRequest,
        GetRepositoryBlobRequest, GetRepositoryBlobsRequest, GetRepositoryPathsRequest,
        GetRepositorySettingsRequest, RepositoryBlobDiffsResponse, RepositoryBlobResponse,
        RepositoryBlobsResponse, RepositoryPathsResponse, RepositoryResponse,
        RepositorySettingsResponse, UpdateRepositorySettingsRequest,
    },
    error::{ConflictError, OptionNotFoundExt, RepositoryError},
    model::{RepositoryOwnerType, RepositorySettings},
    repository::{
        OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
        RepositoryRepositoryImpl,
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

    async fn get_repository_blob_diffs(
        &self,
        request: GetRepositoryBlobDiffsRequest,
    ) -> Result<RepositoryBlobDiffsResponse, RepositoryError>;

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
pub struct RepositoryServiceImpl<G, O, R, D>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    D: DiffClient,
{
    git_client: G,
    org_repo: O,
    repo_repo: R,
    diff_client: D,
}

impl
    RepositoryServiceImpl<
        Git2Client,
        OrganizationRepositoryImpl,
        RepositoryRepositoryImpl,
        DifftClient,
    >
{
    pub fn new(
        git_client: Git2Client,
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
        diff_client: DifftClient,
    ) -> Self {
        Self {
            git_client,
            org_repo,
            repo_repo,
            diff_client,
        }
    }
}

#[crate::instrument_all]
#[async_trait]
impl<G, O, R, D> RepositoryService for RepositoryServiceImpl<G, O, R, D>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
    D: DiffClient,
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
            return Err(RepositoryError::Conflict(ConflictError::new(
                "repository",
                repo_name,
            )));
        }

        let owner_id = match request.owner_type {
            RepositoryOwnerType::User => request.user_id,
            RepositoryOwnerType::Organization => {
                let org = self
                    .org_repo
                    .get(&request.owner_name)
                    .await?
                    .or_not_found("owner", request.owner_name.as_ref())?;
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
        self.git_client
            .get_repo_blob(
                &request.owner_name,
                &request.name,
                &request.ref_name,
                &request.path,
            )
            .await
            .map_err(Into::into)
    }

    async fn get_repository_blobs(
        &self,
        request: GetRepositoryBlobsRequest,
    ) -> Result<RepositoryBlobsResponse, RepositoryError> {
        if request.refs.len() > 1 {
            self.git_client
                .get_repo_blob_at_refs(
                    &request.owner_name,
                    &request.name,
                    &request.paths[0],
                    &request.refs,
                )
                .await
                .map_err(Into::into)
        } else {
            self.git_client
                .get_repo_blobs(
                    &request.owner_name,
                    &request.name,
                    &request.refs[0],
                    &request.paths,
                )
                .await
                .map_err(Into::into)
        }
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

    async fn get_repository_by_id(&self, id: Uuid) -> Result<RepositoryResponse, RepositoryError> {
        let repository = self
            .repo_repo
            .get_by_id(id)
            .await?
            .or_not_found("repository", id)?;

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
            .or_not_found("repository", format!("{}/{}", owner, repo))?;

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

    async fn get_repository_blob_diffs(
        &self,
        request: GetRepositoryBlobDiffsRequest,
    ) -> Result<RepositoryBlobDiffsResponse, RepositoryError> {
        let blobs = self
            .git_client
            .get_repo_blob_at_refs(
                &request.owner_name,
                &request.name,
                &request.path,
                &request.commit_shas,
            )
            .await
            .map_err(RepositoryError::from)?;

        let mut files = Vec::new();
        for blob in blobs.blobs {
            match blob {
                RepositoryBlobResponse::File(f) => files.push(f),
                RepositoryBlobResponse::Folder(_) => {
                    return Err(RepositoryError::NotAFile(request.path.clone()));
                }
            }
        }

        let mut diffs = HashMap::new();
        for (i, ref_name) in request.commit_shas.iter().enumerate() {
            let diff = self
                .diff_client
                .diff_files(files.get(i + 1), files.get(i))
                .await?;
            diffs.insert(ref_name.clone(), diff);
        }

        Ok(RepositoryBlobDiffsResponse { diffs })
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
            .or_not_found("repository", format!("{}/{}", owner, repo))?;
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
            .or_not_found("repository", format!("{}/{}", owner, repo))?;
        Ok(RepositorySettingsResponse {
            commit_filters: settings.commit_filters,
        })
    }
}
