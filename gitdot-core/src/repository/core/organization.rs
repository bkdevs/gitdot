use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{Organization, OrganizationMember, OrganizationRole, UserOrganization},
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

    async fn get_id(&self, org_name: &str) -> Result<Option<Uuid>, DatabaseError>;

    async fn touch_image(&self, org_id: Uuid) -> Result<(), DatabaseError>;

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
        display_name: Option<String>,
    ) -> Result<Option<Organization>, DatabaseError>;

    async fn update_member(
        &self,
        org_name: &str,
        member_id: Uuid,
        role_description: Option<String>,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    async fn list(
        &self,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Organization>, Option<Cursor>), DatabaseError>;

    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, DatabaseError>;

    async fn list_memberships_by_user_id(
        &self,
        user_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<UserOrganization>, Option<Cursor>), DatabaseError>;
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
            "INSERT INTO core.organizations (name, readme) VALUES ($1, $2) RETURNING id, name, created_at, image_updated_at, location, readme, links, display_name, NULL::json AS members",
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
            r#"
            SELECT
                o.id, o.name, o.created_at, o.image_updated_at, o.location, o.readme, o.links, o.display_name,
                COALESCE(
                    (
                        SELECT json_agg(
                            json_build_object(
                                'id', om.id,
                                'user_id', om.user_id,
                                'user_name', u.name,
                                'role', om.role,
                                'role_description', om.role_description,
                                'created_at', om.created_at,
                                'image_updated_at', u.image_updated_at
                            ) ORDER BY om.created_at DESC, om.id DESC
                        )
                        FROM core.organization_members om
                        JOIN core.users u ON u.id = om.user_id
                        WHERE om.organization_id = o.id
                    ),
                    '[]'::json
                ) AS members
            FROM core.organizations o
            WHERE o.name = $1
            "#,
        )
        .bind(org_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(org)
    }

    async fn get_id(&self, org_name: &str) -> Result<Option<Uuid>, DatabaseError> {
        let id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM core.organizations WHERE name = $1")
            .bind(org_name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(id)
    }

    async fn touch_image(&self, org_id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE core.organizations SET image_updated_at = now() WHERE id = $1")
            .bind(org_id)
            .execute(&self.pool)
            .await?;
        Ok(())
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
                RETURNING id, user_id, role, role_description, created_at
            )
            SELECT i.id, i.user_id, i.role, i.role_description, i.created_at, u.name AS user_name, u.image_updated_at
            FROM inserted i
            JOIN core.users u ON i.user_id = u.id
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
            SELECT om.id, om.user_id, om.role, om.role_description, om.created_at, u.name AS user_name, u.image_updated_at
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
        display_name: Option<String>,
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
        if let Some(d) = display_name {
            sep.push("display_name = ").push_bind_unseparated(d);
        }

        builder
            .push(" WHERE name = ")
            .push_bind(org_name)
            .push(" RETURNING id, name, created_at, image_updated_at, location, readme, links, display_name, NULL::json AS members");

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
                RETURNING om.id, om.user_id, om.role, om.role_description, om.created_at
            )
            SELECT updated.id, updated.user_id, updated.role, updated.role_description, updated.created_at, u.name AS user_name, u.image_updated_at
            FROM updated
            JOIN core.users u ON updated.user_id = u.id
            "#,
        )
        .bind(org_name)
        .bind(member_id)
        .bind(role_description)
        .fetch_optional(&self.pool)
        .await?;

        Ok(member)
    }

    async fn list(
        &self,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Organization>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut orgs = sqlx::query_as::<_, Organization>(
            r#"
            SELECT id, name, created_at, image_updated_at, location, readme, links, display_name, NULL::json AS members
            FROM core.organizations
            WHERE ($1::timestamptz IS NULL OR (created_at, id) < ($1, $2))
            ORDER BY created_at DESC, id DESC
            LIMIT $3
            "#,
        )
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if orgs.len() as i64 > limit {
            orgs.pop();
            orgs.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((orgs, next_cursor))
    }

    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, DatabaseError> {
        let orgs = sqlx::query_as::<_, Organization>(
            r#"
            SELECT o.id, o.name, o.created_at, o.image_updated_at, o.location, o.readme, o.links, o.display_name, NULL::json AS members
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

    async fn list_memberships_by_user_id(
        &self,
        user_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<UserOrganization>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut orgs = sqlx::query_as::<_, UserOrganization>(
            r#"
            SELECT o.id, o.name, o.display_name, om.role, om.role_description, om.created_at AS joined_at, o.image_updated_at
            FROM core.organization_members om
            JOIN core.organizations o ON om.organization_id = o.id
            WHERE om.user_id = $1
              AND ($2::timestamptz IS NULL OR (om.created_at, o.id) < ($2, $3))
            ORDER BY om.created_at DESC, o.id DESC
            LIMIT $4
            "#,
        )
        .bind(user_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if orgs.len() as i64 > limit {
            orgs.pop();
            orgs.last().map(|last| Cursor {
                created_at: last.joined_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((orgs, next_cursor))
    }
}
