use async_trait::async_trait;

use crate::dto::{CreateOrganizationRequest, FindOrganizationByNameRequest, FindUserByNameRequest};
use crate::errors::OrganizationError;
use crate::models::Organization;
use crate::repositories::{
    OrganizationRepository, OrganizationRepositoryImpl, UserRepository, UserRepositoryImpl,
};

#[async_trait]
pub trait OrganizationService: Send + Sync + 'static {
    async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, OrganizationError>;
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
    ) -> Result<Organization, OrganizationError> {
        let name = request.name.to_string();

        let find_org_request = FindOrganizationByNameRequest::new(name.clone());
        if self
            .org_repo
            .find_by_name(find_org_request)
            .await?
            .is_some()
        {
            return Err(OrganizationError::Duplicate(name));
        }

        let find_user_request = FindUserByNameRequest::new(name.clone());
        if self
            .user_repo
            .find_by_name(find_user_request)
            .await?
            .is_some()
        {
            return Err(OrganizationError::Duplicate(name));
        }

        let org = self.org_repo.create(request).await?;
        Ok(org)
    }
}
