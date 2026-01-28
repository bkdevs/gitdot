use async_trait::async_trait;

use crate::dto::{
    AddMemberRequest, CreateOrganizationRequest, GetOrganizationRepositoriesRequest,
    GetOrganizationRequest, OrganizationMemberResponse, OrganizationResponse, RepositoryResponse,
};
use crate::error::OrganizationError;
use crate::repository::{
    OrganizationRepository, OrganizationRepositoryImpl, RepositoryRepository,
    RepositoryRepositoryImpl, UserRepository, UserRepositoryImpl,
};

#[async_trait]
pub trait OrganizationService: Send + Sync + 'static {
    async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError>;

    async fn get_organization(
        &self,
        request: GetOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError>;

    async fn add_member(
        &self,
        request: AddMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationError>;

    async fn get_repositories(
        &self,
        request: GetOrganizationRepositoriesRequest,
    ) -> Result<Vec<RepositoryResponse>, OrganizationError>;
}

#[derive(Debug, Clone)]
pub struct OrganizationServiceImpl<O, U, R>
where
    O: OrganizationRepository,
    U: UserRepository,
    R: RepositoryRepository,
{
    org_repo: O,
    user_repo: U,
    repo_repo: R,
}

impl
    OrganizationServiceImpl<
        OrganizationRepositoryImpl,
        UserRepositoryImpl,
        RepositoryRepositoryImpl,
    >
{
    pub fn new(
        org_repo: OrganizationRepositoryImpl,
        user_repo: UserRepositoryImpl,
        repo_repo: RepositoryRepositoryImpl,
    ) -> Self {
        Self {
            org_repo,
            user_repo,
            repo_repo,
        }
    }
}

#[async_trait]
impl<O, U, R> OrganizationService for OrganizationServiceImpl<O, U, R>
where
    O: OrganizationRepository,
    U: UserRepository,
    R: RepositoryRepository,
{
    async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        if self.org_repo.get(&org_name).await?.is_some() {
            return Err(OrganizationError::Duplicate(org_name));
        }
        if self.user_repo.get(&org_name).await?.is_some() {
            return Err(OrganizationError::Duplicate(org_name));
        }

        let org = self.org_repo.create(&org_name, request.owner_id).await?;
        Ok(org.into())
    }

    async fn get_organization(
        &self,
        request: GetOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        let org = self
            .org_repo
            .get(&org_name)
            .await?
            .ok_or_else(|| OrganizationError::NotFound(org_name))?;
        Ok(org.into())
    }

    async fn add_member(
        &self,
        request: AddMemberRequest,
    ) -> Result<OrganizationMemberResponse, OrganizationError> {
        let org_name = request.org_name.to_string();
        let user_name = request.user_name.to_string();

        let member = self
            .org_repo
            .add_member(&org_name, &user_name, request.role)
            .await?;

        match member {
            Some(m) => Ok(m.into()),
            None => {
                if self.org_repo.get(&org_name).await?.is_none() {
                    return Err(OrganizationError::NotFound(org_name));
                }
                if self.user_repo.get(&user_name).await?.is_none() {
                    return Err(OrganizationError::UserNotFound(user_name));
                }
                Err(OrganizationError::MemberAlreadyExists(user_name))
            }
        }
    }

    async fn get_repositories(
        &self,
        request: GetOrganizationRepositoriesRequest,
    ) -> Result<Vec<RepositoryResponse>, OrganizationError> {
        let org_name = request.org_name.to_string();
        self.org_repo
            .get(&org_name)
            .await?
            .ok_or_else(|| OrganizationError::NotFound(org_name.clone()))?;

        let repositories = self.repo_repo.list_by_owner(&org_name).await?;

        Ok(repositories.into_iter().map(|r| r.into()).collect())
    }
}
