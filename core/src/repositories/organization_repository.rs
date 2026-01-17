use async_trait::async_trait;
use sqlx::PgPool;

use crate::dto::{
    AddOrganizationMemberRequest, CreateOrganizationRequest, FindOrganizationByNameRequest,
};
use crate::errors::OrganizationError;
use crate::models::{Organization, OrganizationMember};

#[async_trait]
pub trait OrganizationRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, OrganizationError>;

    async fn find_by_name(
        &self,
        request: FindOrganizationByNameRequest,
    ) -> Result<Option<Organization>, OrganizationError>;

    async fn add_member(
        &self,
        request: AddOrganizationMemberRequest,
    ) -> Result<OrganizationMember, OrganizationError>;
}

#[derive(Debug, Clone)]
pub struct OrganizationRepositoryImpl {
    pool: PgPool,
}

impl OrganizationRepositoryImpl {
    pub fn new(pool: PgPool) -> OrganizationRepositoryImpl {
        OrganizationRepositoryImpl { pool }
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn create(
        &self,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, OrganizationError> {
        let org = sqlx::query_as::<_, Organization>(
            "INSERT INTO organizations (name) VALUES ($1) RETURNING id, name, created_at",
        )
        .bind(request.name.as_ref())
        .fetch_one(&self.pool)
        .await?;

        Ok(org)
    }

    async fn find_by_name(
        &self,
        request: FindOrganizationByNameRequest,
    ) -> Result<Option<Organization>, OrganizationError> {
        let org = sqlx::query_as::<_, Organization>(
            "SELECT id, name, created_at FROM organizations WHERE name = $1",
        )
        .bind(request.name.as_ref())
        .fetch_optional(&self.pool)
        .await?;

        Ok(org)
    }

    async fn add_member(
        &self,
        _request: AddOrganizationMemberRequest,
    ) -> Result<OrganizationMember, OrganizationError> {
        todo!("Implement add_member")
    }
}
