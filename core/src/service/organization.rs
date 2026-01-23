use async_trait::async_trait;

use crate::dto::{CreateOrganizationRequest, GetOrganizationRequest, OrganizationResponse};
use crate::errors::OrganizationError;
use crate::repository::organization::{OrganizationRepository, OrganizationRepositoryImpl};
use crate::repository::user::{UserRepository, UserRepositoryImpl};

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
        Ok(OrganizationResponse::from(org))
    }

    async fn get_organization(
        &self,
        request: GetOrganizationRequest,
    ) -> Result<OrganizationResponse, OrganizationError> {
        let org = self.org_repo.get(&request.org_name).await?;
        Ok(OrganizationResponse::from(org))
    }
}
