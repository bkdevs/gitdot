use async_trait::async_trait;

use crate::dto::{
    AddMemberRequest, CreateOrganizationRequest, GetOrganizationRequest,
    OrganizationMemberResponse, OrganizationResponse,
};
use crate::error::OrganizationError;
use crate::repository::{
    OrganizationRepository, OrganizationRepositoryImpl, UserRepository, UserRepositoryImpl,
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
}

#[derive(Debug, Clone)]
pub struct OrganizationServiceImpl<O, U>
where
    O: OrganizationRepository,
    U: UserRepository,
{
    org_repo: O,
    user_repo: U,
}

impl OrganizationServiceImpl<OrganizationRepositoryImpl, UserRepositoryImpl> {
    pub fn new(org_repo: OrganizationRepositoryImpl, user_repo: UserRepositoryImpl) -> Self {
        Self {
            org_repo: org_repo,
            user_repo: user_repo,
        }
    }
}

#[async_trait]
impl<O, U> OrganizationService for OrganizationServiceImpl<O, U>
where
    O: OrganizationRepository,
    U: UserRepository,
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
        let org = self
            .org_repo
            .get(&org_name)
            .await?
            .ok_or_else(|| OrganizationError::NotFound(org_name))?;

        let user_name = request.user_name.to_string();
        let user = self
            .user_repo
            .get(&user_name)
            .await?
            .ok_or_else(|| OrganizationError::UserNotFound(user_name))?;

        if self.org_repo.is_member(org.id, user.id).await? {
            return Err(OrganizationError::MemberAlreadyExists(user.id));
        }

        let member = self
            .org_repo
            .add_member(org.id, user.id, request.role)
            .await?;

        Ok(member.into())
    }
}
