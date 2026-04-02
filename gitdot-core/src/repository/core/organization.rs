use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{Organization, OrganizationMember, OrganizationRole},
};

#[async_trait]
pub trait OrganizationRepository: Send + Sync + Clone + 'static {
    async fn create(&self, org_name: &str, owner_id: Uuid) -> Result<Organization, DatabaseError>;

    async fn get(&self, org_name: &str) -> Result<Option<Organization>, DatabaseError>;

    async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn add_member(
        &self,
        org_name: &str,
        user_name: &str,
        role: OrganizationRole,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    async fn get_member_role(
        &self,
        org_name: &str,
        user_id: Uuid,
    ) -> Result<Option<OrganizationRole>, DatabaseError>;

    async fn list(&self) -> Result<Vec<Organization>, DatabaseError>;

    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, DatabaseError>;

    async fn list_members(
        &self,
        org_name: &str,
        role: Option<OrganizationRole>,
    ) -> Result<Vec<OrganizationMember>, DatabaseError>;
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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn create(&self, org_name: &str, owner_id: Uuid) -> Result<Organization, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let org = sqlx::query_as::<_, Organization>(
            "INSERT INTO core.organizations (name) VALUES ($1) RETURNING id, name, created_at",
        )
        .bind(org_name)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO core.organization_members (user_id, organization_id, role) VALUES ($1, $2, 'admin')",
        )
        .bind(owner_id)
        .bind(org.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(org)
    }

    async fn get(&self, org_name: &str) -> Result<Option<Organization>, DatabaseError> {
        let org = sqlx::query_as::<_, Organization>(
            "SELECT id, name, created_at FROM core.organizations WHERE name = $1",
        )
        .bind(org_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(org)
    }

    async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError> {
        let result = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM core.organization_members
                WHERE organization_id = $1 AND user_id = $2
            )
            "#,
        )
        .bind(org_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn add_member(
        &self,
        org_name: &str,
        user_name: &str,
        role: OrganizationRole,
    ) -> Result<Option<OrganizationMember>, DatabaseError> {
        let member = sqlx::query_as::<_, OrganizationMember>(
            r#"
            INSERT INTO core.organization_members (user_id, organization_id, role)
            SELECT u.id, o.id, $3
            FROM core.users u, core.organizations o
            WHERE u.name = $1 AND o.name = $2
            ON CONFLICT (user_id, organization_id) DO NOTHING
            RETURNING id, user_id, organization_id, role, created_at
            "#,
        )
        .bind(user_name)
        .bind(org_name)
        .bind(role)
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    async fn get_member_role(
        &self,
        org_name: &str,
        user_id: Uuid,
    ) -> Result<Option<OrganizationRole>, DatabaseError> {
        let role = sqlx::query_scalar::<_, OrganizationRole>(
            r#"
            SELECT om.role
            FROM core.organization_members om
            JOIN core.organizations o ON om.organization_id = o.id
            WHERE o.name = $1 AND om.user_id = $2
            "#,
        )
        .bind(org_name)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(role)
    }

    async fn list(&self) -> Result<Vec<Organization>, DatabaseError> {
        let orgs = sqlx::query_as::<_, Organization>(
            "SELECT id, name, created_at FROM core.organizations ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(orgs)
    }

    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, DatabaseError> {
        let orgs = sqlx::query_as::<_, Organization>(
            r#"
            SELECT o.id, o.name, o.created_at
            FROM core.organizations o
            JOIN core.organization_members om ON o.id = om.organization_id
            WHERE om.user_id = $1
            ORDER BY o.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(orgs)
    }

    async fn list_members(
        &self,
        org_name: &str,
        role: Option<OrganizationRole>,
    ) -> Result<Vec<OrganizationMember>, DatabaseError> {
        let members = sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT om.id, om.user_id, om.organization_id, om.role, om.created_at
            FROM core.organization_members om
            JOIN core.organizations o ON om.organization_id = o.id
            WHERE o.name = $1
            AND ($2::core.organization_role IS NULL OR om.role = $2)
            ORDER BY om.created_at DESC
            "#,
        )
        .bind(org_name)
        .bind(role)
        .fetch_all(&self.pool)
        .await?;

        Ok(members)
    }
}
