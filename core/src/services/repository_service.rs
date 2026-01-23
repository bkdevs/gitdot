use async_trait::async_trait;

use crate::clients::{Git2Client, GitClient};
use crate::dto::{CreateRepositoryRequest, FindOrganizationByNameRequest};
use crate::errors::RepositoryError;
use crate::models::{Repository, RepositoryOwnerType};
use crate::repository::{
    OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
    RepositoryRepositoryImpl,
};

#[async_trait]
pub trait RepositoryService: Send + Sync + 'static {
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<Repository, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct RepositoryServiceImpl<G, O, R>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
{
    git_client: G,
    org_repo: O,
    repo_repo: R,
}

impl RepositoryServiceImpl<Git2Client, OrganizationRepositoryImpl, RepositoryRepositoryImpl> {
    pub fn new(
        git_client: Git2Client,
        org_repo: OrganizationRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
    ) -> Self {
        Self {
            git_client: git_client,
            org_repo: org_repo,
            repo_repo: repo_repo,
        }
    }
}

#[async_trait]
impl<G, O, R> RepositoryService for RepositoryServiceImpl<G, O, R>
where
    G: GitClient,
    O: OrganizationRepository,
    R: RepositoryRepository,
{
    async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<Repository, RepositoryError> {
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
                let find_org_request =
                    FindOrganizationByNameRequest::new(request.owner_name.clone());
                let org = self
                    .org_repo
                    .find_by_name(find_org_request)
                    .await?
                    .ok_or_else(|| RepositoryError::OwnerNotFound(request.owner_name.clone()))?;
                org.id
            }
        };

        // Create git repo first
        self.git_client
            .create_repo(&request.owner_name, &repo_name)
            .await?;

        // Insert into DB, delete git repo on failure
        let owner_name = request.owner_name.clone();
        let repository = match self.repo_repo.create(owner_id, request).await {
            Ok(repo) => repo,
            Err(e) => {
                let _ = self.git_client.delete_repo(&owner_name, &repo_name).await;
                return Err(e.into());
            }
        };

        Ok(repository)
    }
}
