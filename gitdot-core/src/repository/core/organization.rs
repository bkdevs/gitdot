use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{Organization, OrganizationMember, OrganizationRole},
};

#[async_trait]
pub trait OrganizationRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        org_name: &str,
        owner_id: Uuid,
        readme: Option<String>,
    ) -> Result<Organization, DatabaseError>;

    async fn get(&self, org_name: &str) -> Result<Option<Organization>, DatabaseError>;

    async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn add_member(
        &self,
        org_name: &str,
        user_name: &str,
        role: OrganizationRole,
        role_description: Option<String>,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    async fn get_member_role(
        &self,
        org_name: &str,
        user_id: Uuid,
    ) -> Result<Option<OrganizationRole>, DatabaseError>;

    async fn get_member(
        &self,
        org_name: &str,
        member_id: Uuid,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    async fn update(
        &self,
        org_name: &str,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
    ) -> Result<Option<Organization>, DatabaseError>;

    async fn update_member(
        &self,
        org_name: &str,
        member_id: Uuid,
        role_description: Option<String>,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    async fn list(&self) -> Result<Vec<Organization>, DatabaseError>;

    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, DatabaseError>;

    async fn list_members(
        &self,
        org_name: &str,
        role: Option<OrganizationRole>,
    ) -> Result<Vec<OrganizationMember>, DatabaseError>;

    async fn list_memberships_by_user_id(
        &self,
        user_id: Uuid,
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
    async fn create(
        &self,
        org_name: &str,
        owner_id: Uuid,
        readme: Option<String>,
    ) -> Result<Organization, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let org = sqlx::query_as::<_, Organization>(
            "INSERT INTO core.organizations (name, readme) VALUES ($1, $2) RETURNING id, name, created_at, location, readme, links",
        )
        .bind(org_name)
        .bind(readme)
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
            "SELECT id, name, created_at, location, readme, links FROM core.organizations WHERE name = $1",
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
        role_description: Option<String>,
    ) -> Result<Option<OrganizationMember>, DatabaseError> {
        let member = sqlx::query_as::<_, OrganizationMember>(
            r#"
            WITH inserted AS (
                INSERT INTO core.organization_members (user_id, organization_id, role, role_description)
                SELECT u.id, o.id, $3, $4
                FROM core.users u, core.organizations o
                WHERE u.name = $1 AND o.name = $2
                ON CONFLICT (user_id, organization_id) DO NOTHING
                RETURNING id, user_id, organization_id, role, role_description, created_at
            )
            SELECT i.id, i.user_id, i.organization_id, i.role, i.role_description, i.created_at, u.name AS user_name, o.name AS org_name
            FROM inserted i
            JOIN core.users u ON i.user_id = u.id
            JOIN core.organizations o ON i.organization_id = o.id
            "#,
        )
        .bind(user_name)
        .bind(org_name)
        .bind(role)
        .bind(role_description)
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

    async fn get_member(
        &self,
        org_name: &str,
        member_id: Uuid,
    ) -> Result<Option<OrganizationMember>, DatabaseError> {
        let member = sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT om.id, om.user_id, om.organization_id, om.role, om.role_description, om.created_at, u.name AS user_name, o.name AS org_name
            FROM core.organization_members om
            JOIN core.organizations o ON om.organization_id = o.id
            JOIN core.users u ON om.user_id = u.id
            WHERE o.name = $1 AND om.id = $2
            "#,
        )
        .bind(org_name)
        .bind(member_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    async fn update(
        &self,
        org_name: &str,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
    ) -> Result<Option<Organization>, DatabaseError> {
        let mut builder = sqlx::QueryBuilder::new("UPDATE core.organizations SET ");
        let mut sep = builder.separated(", ");

        if let Some(loc) = location {
            sep.push("location = ").push_bind_unseparated(loc);
        }
        if let Some(r) = readme {
            sep.push("readme = ").push_bind_unseparated(r);
        }
        if let Some(l) = links {
            sep.push("links = ").push_bind_unseparated(l);
        }

        builder
            .push(" WHERE name = ")
            .push_bind(org_name)
            .push(" RETURNING id, name, created_at, location, readme, links");

        Ok(builder
            .build_query_as::<Organization>()
            .fetch_optional(&self.pool)
            .await?)
    }

    async fn update_member(
        &self,
        org_name: &str,
        member_id: Uuid,
        role_description: Option<String>,
    ) -> Result<Option<OrganizationMember>, DatabaseError> {
        let member = sqlx::query_as::<_, OrganizationMember>(
            r#"
            WITH updated AS (
                UPDATE core.organization_members om
                SET role_description = COALESCE($3, om.role_description)
                FROM core.organizations o
                WHERE om.organization_id = o.id
                  AND o.name = $1
                  AND om.id = $2
                RETURNING om.id, om.user_id, om.organization_id, om.role, om.role_description, om.created_at
            )
            SELECT updated.id, updated.user_id, updated.organization_id, updated.role, updated.role_description, updated.created_at, u.name AS user_name, o.name AS org_name
            FROM updated
            JOIN core.users u ON updated.user_id = u.id
            JOIN core.organizations o ON updated.organization_id = o.id
            "#,
        )
        .bind(org_name)
        .bind(member_id)
        .bind(role_description)
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    async fn list(&self) -> Result<Vec<Organization>, DatabaseError> {
        let orgs = sqlx::query_as::<_, Organization>(
            "SELECT id, name, created_at, location, readme, links FROM core.organizations ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(orgs)
    }

    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, DatabaseError> {
        let orgs = sqlx::query_as::<_, Organization>(
            r#"
            SELECT o.id, o.name, o.created_at, o.location, o.readme, o.links
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
            SELECT om.id, om.user_id, om.organization_id, om.role, om.role_description, om.created_at, u.name AS user_name, o.name AS org_name
            FROM core.organization_members om
            JOIN core.organizations o ON om.organization_id = o.id
            JOIN core.users u ON om.user_id = u.id
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

    async fn list_memberships_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<OrganizationMember>, DatabaseError> {
        let members = sqlx::query_as::<_, OrganizationMember>(
            r#"
            SELECT om.id, om.user_id, om.organization_id, om.role, om.role_description, om.created_at, u.name AS user_name, o.name AS org_name
            FROM core.organization_members om
            JOIN core.organizations o ON om.organization_id = o.id
            JOIN core.users u ON om.user_id = u.id
            WHERE om.user_id = $1
            ORDER BY om.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(members)
    }
}
