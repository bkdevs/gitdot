use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{Organization, OrganizationMember, OrganizationRole, UserOrganization},
};

/// sqlx data-access layer for organizations.
///
/// Owns the `core.organizations` and `core.organization_members` tables and
/// joins `core.users` to project member details.
#[async_trait]
pub trait OrganizationRepository: Send + Sync + Clone + 'static {
    /// Inserts an organization and enrolls `owner_id` as an `admin` member in
    /// the same transaction. Returns the new org with `members` projected as
    /// `NULL` (not populated by this query).
    async fn create(
        &self,
        org_name: &str,
        owner_id: Uuid,
        readme: Option<String>,
    ) -> Result<Organization, DatabaseError>;

    /// Returns the organization by name with its `members` JSON array populated
    /// (joined from `core.organization_members`/`core.users`, ordered by
    /// `created_at DESC, id DESC`), or `Ok(None)` if no such org.
    async fn get(&self, org_name: &str) -> Result<Option<Organization>, DatabaseError>;

    /// Returns the organization's id by name, or `Ok(None)` if not found.
    async fn get_id(&self, org_name: &str) -> Result<Option<Uuid>, DatabaseError>;

    /// Sets `image_updated_at = now()` for the given org id (no-op if absent).
    async fn touch_image(&self, org_id: Uuid) -> Result<(), DatabaseError>;

    /// Returns whether a `core.organization_members` row exists for the
    /// `(org_id, user_id)` pair.
    async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn follow(&self, org_name: &str, user_id: Uuid) -> Result<Option<Uuid>, DatabaseError>;

    async fn unfollow(&self, org_name: &str, user_id: Uuid) -> Result<bool, DatabaseError>;

    /// Inserts a membership resolving `user_name`/`org_name` to ids, with
    /// `ON CONFLICT (user_id, organization_id) DO NOTHING`. Returns the new
    /// member (joined to `core.users` for `user_name`), or `Ok(None)` when the
    /// user/org name doesn't resolve or the membership already exists.
    async fn add_member(
        &self,
        org_name: &str,
        user_name: &str,
        role: OrganizationRole,
        role_description: Option<String>,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    /// Returns the member's role for the given org name and user id (joins
    /// `core.organization_members` to `core.organizations`), or `Ok(None)` if
    /// the user is not a member.
    async fn get_member_role(
        &self,
        org_name: &str,
        user_id: Uuid,
    ) -> Result<Option<OrganizationRole>, DatabaseError>;

    /// Returns the membership identified by org name and member id (joined to
    /// `core.users` for `user_name`), or `Ok(None)` if not found.
    async fn get_member(
        &self,
        org_name: &str,
        member_id: Uuid,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    /// Updates only the provided (`Some`) fields on `core.organizations` via a
    /// dynamic `QueryBuilder` and `RETURNING`s the row with `members` as `NULL`.
    /// Returns `Ok(None)` if no org matches `org_name`.
    async fn update(
        &self,
        org_name: &str,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
        display_name: Option<String>,
    ) -> Result<Option<Organization>, DatabaseError>;

    /// Updates a member's `role_description` via `COALESCE($3, role_description)`
    /// (a `None` leaves the existing value unchanged), scoped by org name and
    /// member id. Returns the updated member, or `Ok(None)` if not found.
    async fn update_member(
        &self,
        org_name: &str,
        member_id: Uuid,
        role_description: Option<String>,
    ) -> Result<Option<OrganizationMember>, DatabaseError>;

    /// Lists organizations newest-first (`ORDER BY created_at DESC, id DESC`)
    /// with keyset cursor pagination. Returns the page plus an
    /// `Option<Cursor>` for the next page (`None` when exhausted). `members` is
    /// projected as `NULL`.
    async fn list(
        &self,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Organization>, Option<Cursor>), DatabaseError>;

    /// Lists all organizations the user is a member of (join on
    /// `core.organization_members`), ordered by org `created_at DESC`.
    /// `members` is projected as `NULL`.
    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, DatabaseError>;

    /// Lists the user's memberships as `UserOrganization` rows (org fields plus
    /// the member's role and `joined_at`), newest-join first
    /// (`ORDER BY om.created_at DESC, o.id DESC`) with keyset cursor pagination.
    /// Returns the page plus the next cursor (`None` when exhausted); the cursor
    /// is keyed on membership `created_at` (join time).
    async fn list_memberships_by_user_id(
        &self,
        user_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<UserOrganization>, Option<Cursor>), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgOrganizationRepository {
    pool: PgPool,
}

impl PgOrganizationRepository {
    pub fn new(pool: PgPool) -> PgOrganizationRepository {
        PgOrganizationRepository { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl OrganizationRepository for PgOrganizationRepository {
    async fn create(
        &self,
        org_name: &str,
        owner_id: Uuid,
        readme: Option<String>,
    ) -> Result<Organization, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let org = sqlx::query_as::<_, Organization>(
            "INSERT INTO core.organizations (name, readme) VALUES ($1, $2) RETURNING id, name, created_at, image_updated_at, location, readme, links, display_name, 0::bigint AS followers, false AS user_follow, NULL::json AS members",
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
                (SELECT COUNT(*)::bigint FROM core.organization_followers ofl WHERE ofl.organization_id = o.id) AS followers,
                false AS user_follow,
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

    async fn follow(&self, org_name: &str, user_id: Uuid) -> Result<Option<Uuid>, DatabaseError> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO core.organization_followers (user_id, organization_id)
            SELECT $2, o.id
            FROM core.organizations o
            WHERE o.name = $1
            ON CONFLICT (user_id, organization_id) DO NOTHING
            RETURNING id
            "#,
        )
        .bind(org_name)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(id)
    }

    async fn unfollow(&self, org_name: &str, user_id: Uuid) -> Result<bool, DatabaseError> {
        let result = sqlx::query(
            r#"
            DELETE FROM core.organization_followers ofl
            USING core.organizations o
            WHERE ofl.organization_id = o.id
              AND o.name = $1
              AND ofl.user_id = $2
            "#,
        )
        .bind(org_name)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
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
            .push(" RETURNING id, name, created_at, image_updated_at, location, readme, links, display_name, 0::bigint AS followers, false AS user_follow, NULL::json AS members");

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
            SELECT id, name, created_at, image_updated_at, location, readme, links, display_name, 0::bigint AS followers, false AS user_follow, NULL::json AS members
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
            SELECT o.id, o.name, o.created_at, o.image_updated_at, o.location, o.readme, o.links, o.display_name, 0::bigint AS followers, false AS user_follow, NULL::json AS members
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

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{OrganizationRepository, OrganizationRole, PgOrganizationRepository};
    use crate::repository::test_common::{insert_membership_at, insert_org_at, insert_user};

    #[sqlx::test]
    async fn create_persists_org_and_owner_admin_membership(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;

        let org = repo
            .create("acme", owner, Some("hello".to_string()))
            .await
            .unwrap();
        assert_eq!(org.name, "acme");
        assert_eq!(org.readme.as_deref(), Some("hello"));

        // The creator is enrolled as an admin member in the same transaction.
        assert!(repo.is_member(org.id, owner).await.unwrap());
        assert_eq!(
            repo.get_member_role("acme", owner).await.unwrap(),
            Some(OrganizationRole::Admin)
        );
    }

    #[sqlx::test]
    async fn get_returns_org_with_members(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        repo.create("acme", owner, None).await.unwrap();

        let org = repo.get("acme").await.unwrap().expect("org exists");
        let members = org.members.expect("members projected");
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].user_name, "owner");
        assert_eq!(members[0].role, OrganizationRole::Admin);

        assert!(repo.get("missing").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn get_id_round_trips(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        let org = repo.create("acme", owner, None).await.unwrap();

        assert_eq!(repo.get_id("acme").await.unwrap(), Some(org.id));
        assert!(repo.get_id("missing").await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn touch_image_advances_timestamp(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let id = Uuid::new_v4();
        // Seed with an old image timestamp so the touch is observable.
        sqlx::query(
            "INSERT INTO core.organizations (id, name, image_updated_at)
             VALUES ($1, $2, NOW() - INTERVAL '1 day')",
        )
        .bind(id)
        .bind("acme")
        .execute(&pool)
        .await
        .unwrap();

        let before = repo.get("acme").await.unwrap().unwrap().image_updated_at;
        repo.touch_image(id).await.unwrap();
        let after = repo.get("acme").await.unwrap().unwrap().image_updated_at;
        assert!(after > before, "expected {after} > {before}");
    }

    #[sqlx::test]
    async fn is_member_reflects_membership(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        let outsider = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        insert_user(&pool, outsider, "outsider").await;
        let org = repo.create("acme", owner, None).await.unwrap();

        assert!(repo.is_member(org.id, owner).await.unwrap());
        assert!(!repo.is_member(org.id, outsider).await.unwrap());
    }

    #[sqlx::test]
    async fn add_member_inserts_then_is_idempotent(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        insert_user(&pool, bob, "bob").await;
        repo.create("acme", owner, None).await.unwrap();

        let member = repo
            .add_member(
                "acme",
                "bob",
                OrganizationRole::Member,
                Some("maintainer".to_string()),
            )
            .await
            .unwrap()
            .expect("member inserted");
        assert_eq!(member.user_id, bob);
        assert_eq!(member.user_name, "bob");
        assert_eq!(member.role, OrganizationRole::Member);
        assert_eq!(member.role_description.as_deref(), Some("maintainer"));

        // A second add for the same (user, org) hits ON CONFLICT DO NOTHING.
        assert!(
            repo.add_member("acme", "bob", OrganizationRole::Member, None)
                .await
                .unwrap()
                .is_none()
        );

        // A nonexistent user produces no row to insert.
        assert!(
            repo.add_member("acme", "ghost", OrganizationRole::Member, None)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn get_member_role_resolves_by_org_and_user(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        let outsider = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        insert_user(&pool, outsider, "outsider").await;
        repo.create("acme", owner, None).await.unwrap();

        assert_eq!(
            repo.get_member_role("acme", owner).await.unwrap(),
            Some(OrganizationRole::Admin)
        );
        assert!(
            repo.get_member_role("acme", outsider)
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            repo.get_member_role("missing", owner)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn get_member_fetches_by_member_id(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        insert_user(&pool, bob, "bob").await;
        repo.create("acme", owner, None).await.unwrap();
        let added = repo
            .add_member("acme", "bob", OrganizationRole::Member, None)
            .await
            .unwrap()
            .unwrap();

        let member = repo
            .get_member("acme", added.id)
            .await
            .unwrap()
            .expect("member found");
        assert_eq!(member.id, added.id);
        assert_eq!(member.user_name, "bob");

        assert!(
            repo.get_member("acme", Uuid::new_v4())
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn update_changes_only_provided_fields(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        repo.create("acme", owner, None).await.unwrap();

        let updated = repo
            .update(
                "acme",
                Some("Earth".to_string()),
                Some("readme".to_string()),
                Some(vec!["https://example.com".to_string()]),
                Some("Acme Inc".to_string()),
            )
            .await
            .unwrap()
            .expect("org updated");
        assert_eq!(updated.location.as_deref(), Some("Earth"));
        assert_eq!(updated.readme.as_deref(), Some("readme"));
        assert_eq!(updated.links, vec!["https://example.com".to_string()]);
        assert_eq!(updated.display_name.as_deref(), Some("Acme Inc"));

        // A partial update touches only the provided field.
        let again = repo
            .update("acme", Some("Mars".to_string()), None, None, None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(again.location.as_deref(), Some("Mars"));
        assert_eq!(again.readme.as_deref(), Some("readme"));
        assert_eq!(again.display_name.as_deref(), Some("Acme Inc"));

        // Updating a missing org yields no row.
        assert!(
            repo.update("missing", Some("x".to_string()), None, None, None)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn update_member_coalesces_role_description(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        insert_user(&pool, bob, "bob").await;
        repo.create("acme", owner, None).await.unwrap();
        let added = repo
            .add_member(
                "acme",
                "bob",
                OrganizationRole::Member,
                Some("first".to_string()),
            )
            .await
            .unwrap()
            .unwrap();

        let updated = repo
            .update_member("acme", added.id, Some("second".to_string()))
            .await
            .unwrap()
            .expect("member updated");
        assert_eq!(updated.role_description.as_deref(), Some("second"));

        // A None description is coalesced to the existing value, not cleared.
        let unchanged = repo
            .update_member("acme", added.id, None)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(unchanged.role_description.as_deref(), Some("second"));

        assert!(
            repo.update_member("acme", Uuid::new_v4(), Some("x".to_string()))
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn list_paginates_newest_first(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let now = Utc::now();
        let (o1, o2, o3) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        insert_org_at(&pool, o1, "first", now - Duration::days(3)).await;
        insert_org_at(&pool, o2, "second", now - Duration::days(2)).await;
        insert_org_at(&pool, o3, "third", now - Duration::days(1)).await;

        let (page, cursor) = repo.list(None, 2).await.unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].name, "third");
        assert_eq!(page[1].name, "second");
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo.list(Some(cursor), 2).await.unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].name, "first");
        assert!(cursor.is_none());
    }

    #[sqlx::test]
    async fn list_by_user_id_returns_member_orgs(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let owner = Uuid::new_v4();
        let outsider = Uuid::new_v4();
        insert_user(&pool, owner, "owner").await;
        insert_user(&pool, outsider, "outsider").await;
        repo.create("acme", owner, None).await.unwrap();
        repo.create("globex", owner, None).await.unwrap();
        repo.create("initech", outsider, None).await.unwrap();

        let owner_orgs = repo.list_by_user_id(owner).await.unwrap();
        let mut names: Vec<_> = owner_orgs.iter().map(|o| o.name.clone()).collect();
        names.sort();
        assert_eq!(names, vec!["acme".to_string(), "globex".to_string()]);

        let outsider_orgs = repo.list_by_user_id(outsider).await.unwrap();
        assert_eq!(outsider_orgs.len(), 1);
        assert_eq!(outsider_orgs[0].name, "initech");

        assert!(
            repo.list_by_user_id(Uuid::new_v4())
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[sqlx::test]
    async fn list_memberships_paginates_newest_first(pool: PgPool) {
        let repo = PgOrganizationRepository::new(pool.clone());
        let user = Uuid::new_v4();
        insert_user(&pool, user, "member").await;

        let now = Utc::now();
        let (o1, o2, o3) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        insert_org_at(&pool, o1, "first", now).await;
        insert_org_at(&pool, o2, "second", now).await;
        insert_org_at(&pool, o3, "third", now).await;
        insert_membership_at(
            &pool,
            user,
            o1,
            OrganizationRole::Admin,
            now - Duration::days(3),
        )
        .await;
        insert_membership_at(
            &pool,
            user,
            o2,
            OrganizationRole::Member,
            now - Duration::days(2),
        )
        .await;
        insert_membership_at(
            &pool,
            user,
            o3,
            OrganizationRole::Admin,
            now - Duration::days(1),
        )
        .await;

        // Ordered by join time (membership created_at) descending.
        let (page, cursor) = repo
            .list_memberships_by_user_id(user, None, 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].name, "third");
        assert_eq!(page[0].role, OrganizationRole::Admin);
        assert_eq!(page[1].name, "second");
        assert_eq!(page[1].role, OrganizationRole::Member);
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo
            .list_memberships_by_user_id(user, Some(cursor), 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].name, "first");
        assert!(cursor.is_none());
    }
}
