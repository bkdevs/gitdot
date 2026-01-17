use async_trait::async_trait;
use sqlx::PgPool;

use crate::dto::organization_dto::CreateOrganizationRequest;
use crate::errors::organization_error::OrganizationError;
use crate::models::organization::Organization;
use crate::repositories::organization_repository::{
    OrganizationRepository, OrganizationRepositoryImpl,
};

#[async_trait]
pub trait OrganizationService: Send + Sync + 'static {
    async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, OrganizationError>;
}

#[derive(Debug, Clone)]
pub struct OrganizationServiceImpl<R: OrganizationRepository> {
    org_repo: R,
}

impl OrganizationServiceImpl<OrganizationRepositoryImpl> {
    pub fn new(pool: PgPool) -> Self {
        Self {
            org_repo: OrganizationRepositoryImpl::new(pool),
        }
    }
}

#[async_trait]
impl<R: OrganizationRepository> OrganizationService for OrganizationServiceImpl<R> {
    async fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, OrganizationError> {
        // TODO: verify any existing user or org with the name

        let org = self.org_repo.create(request).await?;

        Ok(org)
    }
}
