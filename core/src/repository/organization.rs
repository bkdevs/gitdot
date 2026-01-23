use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::Organization;

#[async_trait]
pub trait OrganizationRepository: Send + Sync + Clone + 'static {
    async fn create(&self, org_name: &str, owner_id: Uuid) -> Result<Organization, Error>;

    async fn get(&self, org_name: &str) -> Result<Option<Organization>, Error>;
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
    async fn create(&self, org_name: &str, owner_id: Uuid) -> Result<Organization, Error> {
        let mut tx = self.pool.begin().await?;

        let org = sqlx::query_as::<_, Organization>(
            "INSERT INTO organizations (name) VALUES ($1) RETURNING id, name, created_at",
        )
        .bind(org_name)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO organization_members (user_id, organization_id, role) VALUES ($1, $2, 'admin')",
        )
        .bind(owner_id)
        .bind(org.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(org)
    }

    async fn get(&self, org_name: &str) -> Result<Option<Organization>, Error> {
        let org = sqlx::query_as::<_, Organization>(
            "SELECT id, name, created_at FROM organizations WHERE name = $1",
        )
        .bind(org_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(org)
    }
}
