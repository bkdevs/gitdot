use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{
        AuthProvider, CommitFilter, Repository, RepositoryOwnerType, RepositoryStar,
        RepositoryVisibility, User, UserEmail,
    },
};

#[async_trait]
pub trait RepositoryRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
        description: Option<String>,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Repository, DatabaseError>;

    async fn get(
        &self,
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError>;

    async fn get_id(&self, owner: &str, repo: &str) -> Result<Option<Uuid>, DatabaseError>;

    async fn get_by_id(
        &self,
        id: Uuid,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError>;

    async fn list_by_owner(
        &self,
        owner_name: &str,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError>;

    async fn list_latest(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError>;

    async fn list_trending(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError>;

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn update(
        &self,
        id: Uuid,
        description: Option<String>,
        readonly: Option<bool>,
    ) -> Result<Option<Repository>, DatabaseError>;

    async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, DatabaseError>;

    async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(User, DateTime<Utc>)>, DatabaseError>;

    async fn list_commit_filters(
        &self,
        repository_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<CommitFilter>, Option<Cursor>), DatabaseError>;

    async fn create_commit_filter(
        &self,
        repository_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<CommitFilter, DatabaseError>;

    async fn update_commit_filter(
        &self,
        filter_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Option<CommitFilter>, DatabaseError>;

    async fn delete_commit_filter(&self, filter_id: Uuid) -> Result<bool, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct RepositoryRepositoryImpl {
    pool: PgPool,
}

impl RepositoryRepositoryImpl {
    pub fn new(pool: PgPool) -> RepositoryRepositoryImpl {
        RepositoryRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl RepositoryRepository for RepositoryRepositoryImpl {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
        description: Option<String>,
        readonly: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Repository, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            WITH inserted AS (
                INSERT INTO core.repositories (name, owner_id, owner_type, visibility, description, readonly, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, NOW()))
                RETURNING id, name, owner_id, owner_type, visibility, description, stars, readonly, created_at
            )
            SELECT i.id, i.name, i.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   i.owner_type, i.visibility, i.description, i.stars, i.readonly, i.created_at,
                   false AS user_star
            FROM inserted i
            LEFT JOIN core.users u
              ON i.owner_id = u.id AND i.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON i.owner_id = o.id AND i.owner_type = 'organization'
            "#,
        )
        .bind(name)
        .bind(owner_id)
        .bind(owner_type)
        .bind(visibility)
        .bind(description)
        .bind(readonly)
        .bind(created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get(
        &self,
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars s WHERE s.repository_id = r.id AND s.user_id = $3) AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.name = $2
              AND r.owner_id IN (
                SELECT id FROM core.users         WHERE name = $1
                UNION ALL
                SELECT id FROM core.organizations WHERE name = $1
              )
            "#,
        )
        .bind(owner)
        .bind(repo)
        .bind(viewer_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get_id(&self, owner: &str, repo: &str) -> Result<Option<Uuid>, DatabaseError> {
        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT r.id
            FROM core.repositories r
            WHERE r.name = $2
              AND r.owner_id IN (
                SELECT id FROM core.users         WHERE name = $1
                UNION ALL
                SELECT id FROM core.organizations WHERE name = $1
              )
            "#,
        )
        .bind(owner)
        .bind(repo)
        .fetch_optional(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_by_id(
        &self,
        id: Uuid,
        viewer_id: Option<Uuid>,
    ) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars s WHERE s.repository_id = r.id AND s.user_id = $2) AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.id = $1
            "#,
        )
        .bind(id)
        .bind(viewer_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn list_by_owner(
        &self,
        owner_name: &str,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut repositories = sqlx::query_as::<_, Repository>(
            r#"
            WITH viewer_orgs AS (
                SELECT organization_id FROM core.organization_members WHERE user_id = $5
            )
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars s WHERE s.repository_id = r.id AND s.user_id = $5) AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.owner_id IN (
                SELECT id FROM core.users         WHERE name = $1
                UNION ALL
                SELECT id FROM core.organizations WHERE name = $1
            )
              AND (
                  r.visibility = 'public'
                  OR (r.owner_type = 'user' AND r.owner_id = $5)
                  OR (r.owner_type = 'organization'
                      AND r.owner_id IN (SELECT organization_id FROM viewer_orgs))
              )
              AND ($2::timestamptz IS NULL OR (r.created_at, r.id) < ($2, $3))
            ORDER BY r.created_at DESC, r.id DESC
            LIMIT $4
            "#,
        )
        .bind(owner_name)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .bind(viewer_id)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if repositories.len() as i64 > limit {
            repositories.pop();
            repositories.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((repositories, next_cursor))
    }

    async fn list_latest(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError> {
        let repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   FALSE AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.visibility = 'public'
            ORDER BY r.created_at DESC, r.id DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(repositories)
    }

    async fn list_trending(&self, limit: i64) -> Result<Vec<Repository>, DatabaseError> {
        let repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   FALSE AS user_star
            FROM core.repositories r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            WHERE r.visibility = 'public'
            ORDER BY r.stars DESC, r.created_at DESC, r.id DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(repositories)
    }

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM core.repositories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update(
        &self,
        id: Uuid,
        description: Option<String>,
        readonly: Option<bool>,
    ) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            WITH updated AS (
                UPDATE core.repositories r
                SET description = COALESCE($2, r.description),
                    readonly    = COALESCE($3, r.readonly)
                WHERE r.id = $1
                RETURNING r.*
            )
            SELECT r.id, r.name, r.owner_id, COALESCE(u.name, o.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   FALSE AS user_star
            FROM updated r
            LEFT JOIN core.users u
              ON r.owner_id = u.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations o
              ON r.owner_id = o.id AND r.owner_type = 'organization'
            "#,
        )
        .bind(id)
        .bind(description)
        .bind(readonly)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let star = sqlx::query_as::<_, RepositoryStar>(
            r#"
            INSERT INTO core.stars (user_id, repository_id)
            VALUES ($1, $2)
            ON CONFLICT (user_id, repository_id) DO NOTHING
            RETURNING id, user_id, repository_id, created_at
            "#,
        )
        .bind(user_id)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        if star.is_some() {
            sqlx::query("UPDATE core.repositories SET stars = stars + 1 WHERE id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(star)
    }

    async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let deleted =
            sqlx::query("DELETE FROM core.stars WHERE user_id = $1 AND repository_id = $2")
                .bind(user_id)
                .bind(id)
                .execute(&mut *tx)
                .await?
                .rows_affected();

        if deleted > 0 {
            sqlx::query(
                "UPDATE core.repositories SET stars = GREATEST(stars - 1, 0) WHERE id = $1",
            )
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(deleted > 0)
    }

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(User, DateTime<Utc>)>, DatabaseError> {
        #[derive(FromRow)]
        struct StarredUserRow {
            id: Uuid,
            name: String,
            provider: AuthProvider,
            created_at: DateTime<Utc>,
            image_updated_at: DateTime<Utc>,
            display_name: Option<String>,
            location: Option<String>,
            readme: Option<String>,
            links: Vec<String>,
            starred_at: DateTime<Utc>,
        }

        let rows = sqlx::query_as::<_, StarredUserRow>(
            r#"
            SELECT u.id, u.name, u.provider, u.created_at, u.image_updated_at, u.display_name,
                   u.location, u.readme, u.links,
                   s.created_at AS starred_at
            FROM core.stars s
            JOIN core.users u ON s.user_id = u.id
            WHERE s.repository_id = $1
            ORDER BY s.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(repository_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        if rows.is_empty() {
            return Ok(Vec::new());
        }

        let user_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let emails = sqlx::query_as::<_, UserEmail>(
            r#"
            SELECT id, user_id, email, is_primary, is_verified, created_at
            FROM core.user_emails
            WHERE user_id = ANY($1)
            ORDER BY is_primary DESC, created_at ASC
            "#,
        )
        .bind(&user_ids)
        .fetch_all(&self.pool)
        .await?;

        let mut emails_by_user: std::collections::HashMap<Uuid, Vec<UserEmail>> =
            std::collections::HashMap::new();
        for e in emails {
            emails_by_user.entry(e.user_id).or_default().push(e);
        }

        Ok(rows
            .into_iter()
            .map(|r| {
                let user = User {
                    id: r.id,
                    name: r.name,
                    provider: r.provider,
                    display_name: r.display_name,
                    location: r.location,
                    readme: r.readme,
                    links: r.links,
                    created_at: r.created_at,
                    image_updated_at: r.image_updated_at,
                    emails: emails_by_user.remove(&r.id).unwrap_or_default(),
                };
                (user, r.starred_at)
            })
            .collect())
    }

    async fn list_commit_filters(
        &self,
        repository_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<CommitFilter>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut filters = sqlx::query_as::<_, CommitFilter>(
            r#"
            SELECT id, repository_id, name, authors, tags, paths, created_at, updated_at
            FROM core.commit_filters
            WHERE repository_id = $1
              AND ($2::timestamptz IS NULL OR (created_at, id) < ($2, $3))
            ORDER BY created_at DESC, id DESC
            LIMIT $4
            "#,
        )
        .bind(repository_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if filters.len() as i64 > limit {
            filters.pop();
            filters.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((filters, next_cursor))
    }

    async fn create_commit_filter(
        &self,
        repository_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<CommitFilter, DatabaseError> {
        let filter = sqlx::query_as::<_, CommitFilter>(
            r#"
            INSERT INTO core.commit_filters (repository_id, name, authors, tags, paths)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, repository_id, name, authors, tags, paths, created_at, updated_at
            "#,
        )
        .bind(repository_id)
        .bind(name)
        .bind(authors)
        .bind(tags)
        .bind(paths)
        .fetch_one(&self.pool)
        .await?;

        Ok(filter)
    }

    async fn update_commit_filter(
        &self,
        filter_id: Uuid,
        name: &str,
        authors: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        paths: Option<Vec<String>>,
    ) -> Result<Option<CommitFilter>, DatabaseError> {
        let filter = sqlx::query_as::<_, CommitFilter>(
            r#"
            UPDATE core.commit_filters
            SET name = $2,
                authors = $3,
                tags = $4,
                paths = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, repository_id, name, authors, tags, paths, created_at, updated_at
            "#,
        )
        .bind(filter_id)
        .bind(name)
        .bind(authors)
        .bind(tags)
        .bind(paths)
        .fetch_optional(&self.pool)
        .await?;

        Ok(filter)
    }

    async fn delete_commit_filter(&self, filter_id: Uuid) -> Result<bool, DatabaseError> {
        let result = sqlx::query("DELETE FROM core.commit_filters WHERE id = $1")
            .bind(filter_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{
        Repository, RepositoryOwnerType, RepositoryRepository, RepositoryRepositoryImpl,
        RepositoryVisibility,
    };
    use crate::{
        model::OrganizationRole,
        repository::test_common::{
            insert_filter_at, insert_membership_at, insert_org, insert_org_repo, insert_star_at,
            insert_user, insert_user_repo,
        },
    };

    // The common case: a user-owned, non-readonly repo with no description.
    async fn make_repo(
        repo: &RepositoryRepositoryImpl,
        name: &str,
        owner: Uuid,
        visibility: RepositoryVisibility,
        created_at: Option<DateTime<Utc>>,
    ) -> Repository {
        repo.create(
            name,
            owner,
            &RepositoryOwnerType::User,
            &visibility,
            None,
            false,
            created_at,
        )
        .await
        .unwrap()
    }

    #[sqlx::test]
    async fn create_returns_repo_with_owner_name(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;

        let created = repo
            .create(
                "proj",
                alice,
                &RepositoryOwnerType::User,
                &RepositoryVisibility::Public,
                Some("a project".to_string()),
                false,
                None,
            )
            .await
            .unwrap();
        assert_eq!(created.name, "proj");
        assert_eq!(created.owner_id, alice);
        assert_eq!(created.owner_name, "alice");
        assert_eq!(created.owner_type, RepositoryOwnerType::User);
        assert_eq!(created.visibility, RepositoryVisibility::Public);
        assert_eq!(created.description.as_deref(), Some("a project"));
        assert_eq!(created.stars, 0);
        assert!(!created.user_star);
        assert!(!created.readonly);
    }

    #[sqlx::test]
    async fn create_supports_organization_owner(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let org = Uuid::new_v4();
        insert_org(&pool, org, "acme").await;

        let created = repo
            .create(
                "proj",
                org,
                &RepositoryOwnerType::Organization,
                &RepositoryVisibility::Public,
                None,
                false,
                None,
            )
            .await
            .unwrap();
        // owner_name is resolved via the organizations join branch.
        assert_eq!(created.owner_name, "acme");
        assert_eq!(created.owner_type, RepositoryOwnerType::Organization);
    }

    #[sqlx::test]
    async fn get_lookups_round_trip(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;

        let by_name = repo
            .get("alice", "proj", None)
            .await
            .unwrap()
            .expect("found");
        assert_eq!(by_name.id, created.id);
        assert_eq!(
            repo.get_id("alice", "proj").await.unwrap(),
            Some(created.id)
        );
        let by_id = repo
            .get_by_id(created.id, None)
            .await
            .unwrap()
            .expect("found");
        assert_eq!(by_id.name, "proj");

        assert!(repo.get("alice", "missing", None).await.unwrap().is_none());
        assert!(repo.get("ghost", "proj", None).await.unwrap().is_none());
        assert!(repo.get_id("alice", "missing").await.unwrap().is_none());
        assert!(
            repo.get_by_id(Uuid::new_v4(), None)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn list_by_owner_paginates_newest_first(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        let now = Utc::now();
        make_repo(
            &repo,
            "first",
            alice,
            RepositoryVisibility::Public,
            Some(now - Duration::days(3)),
        )
        .await;
        make_repo(
            &repo,
            "second",
            alice,
            RepositoryVisibility::Public,
            Some(now - Duration::days(2)),
        )
        .await;
        make_repo(
            &repo,
            "third",
            alice,
            RepositoryVisibility::Public,
            Some(now - Duration::days(1)),
        )
        .await;
        // A repo owned by someone else must not appear in alice's listing.
        make_repo(&repo, "other", bob, RepositoryVisibility::Public, Some(now)).await;

        let (page, cursor) = repo.list_by_owner("alice", None, None, 2).await.unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].name, "third");
        assert_eq!(page[1].name, "second");
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo
            .list_by_owner("alice", None, Some(cursor), 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].name, "first");
        assert!(cursor.is_none());
    }

    #[sqlx::test]
    async fn list_by_owner_filters_user_repos_by_viewer(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        insert_user_repo(&pool, Uuid::new_v4(), "pub", alice, "public").await;
        insert_user_repo(&pool, Uuid::new_v4(), "priv", alice, "private").await;

        // Anonymous viewer sees only the public repo.
        let (rows, _) = repo.list_by_owner("alice", None, None, 20).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "pub");

        // The owner sees their own private repo too.
        let (rows, _) = repo
            .list_by_owner("alice", Some(alice), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);

        // A different user sees only the public repo.
        let (rows, _) = repo
            .list_by_owner("alice", Some(bob), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[sqlx::test]
    async fn list_by_owner_shows_org_private_repos_to_members(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let member = Uuid::new_v4();
        let outsider = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        insert_user(&pool, member, "member").await;
        insert_user(&pool, outsider, "outsider").await;
        insert_org(&pool, org_id, "acme").await;
        insert_membership_at(&pool, member, org_id, OrganizationRole::Member, Utc::now()).await;

        insert_org_repo(&pool, Uuid::new_v4(), "pub", org_id, "public").await;
        insert_org_repo(&pool, Uuid::new_v4(), "priv", org_id, "private").await;

        // A member sees the org's private repo.
        let (rows, _) = repo
            .list_by_owner("acme", Some(member), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);

        // A non-member and an anonymous viewer see only the public one.
        let (rows, _) = repo
            .list_by_owner("acme", Some(outsider), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        let (rows, _) = repo.list_by_owner("acme", None, None, 20).await.unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[sqlx::test]
    async fn list_latest_returns_public_only_newest_first(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let now = Utc::now();
        make_repo(
            &repo,
            "older",
            alice,
            RepositoryVisibility::Public,
            Some(now - Duration::days(2)),
        )
        .await;
        make_repo(
            &repo,
            "newer",
            alice,
            RepositoryVisibility::Public,
            Some(now - Duration::days(1)),
        )
        .await;
        make_repo(
            &repo,
            "secret",
            alice,
            RepositoryVisibility::Private,
            Some(now),
        )
        .await;

        let repos = repo.list_latest(10).await.unwrap();
        let names: Vec<_> = repos.iter().map(|r| r.name.as_str()).collect();
        // Private repos are excluded; public ones come back newest first.
        assert_eq!(names, vec!["newer", "older"]);
    }

    #[sqlx::test]
    async fn list_trending_orders_by_stars(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let carol = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        insert_user(&pool, carol, "carol").await;
        let popular = make_repo(&repo, "popular", alice, RepositoryVisibility::Public, None).await;
        let quiet = make_repo(&repo, "quiet", alice, RepositoryVisibility::Public, None).await;
        let secret = make_repo(&repo, "secret", alice, RepositoryVisibility::Private, None).await;

        repo.star(popular.id, bob).await.unwrap();
        repo.star(popular.id, carol).await.unwrap();
        repo.star(quiet.id, bob).await.unwrap();
        repo.star(secret.id, bob).await.unwrap();

        let repos = repo.list_trending(10).await.unwrap();
        let names: Vec<_> = repos.iter().map(|r| r.name.as_str()).collect();
        // Public repos ordered by star count desc; the private repo is excluded.
        assert_eq!(names, vec!["popular", "quiet"]);
    }

    #[sqlx::test]
    async fn delete_removes_repo(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;

        repo.delete(created.id).await.unwrap();
        assert!(repo.get_by_id(created.id, None).await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn update_sets_description(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let created = repo
            .create(
                "proj",
                alice,
                &RepositoryOwnerType::User,
                &RepositoryVisibility::Public,
                Some("old".to_string()),
                false,
                None,
            )
            .await
            .unwrap();

        let updated = repo
            .update(created.id, Some("new".to_string()), None)
            .await
            .unwrap()
            .expect("updated");
        assert_eq!(updated.description.as_deref(), Some("new"));

        assert!(
            repo.update(Uuid::new_v4(), Some("x".to_string()), None)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn update_clears_readonly_flag(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let created = repo
            .create(
                "proj",
                alice,
                &RepositoryOwnerType::User,
                &RepositoryVisibility::Public,
                None,
                true,
                None,
            )
            .await
            .unwrap();

        let updated = repo
            .update(created.id, None, Some(false))
            .await
            .unwrap()
            .expect("updated");
        assert!(!updated.readonly);

        assert!(
            repo.update(Uuid::new_v4(), None, Some(false))
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn star_is_idempotent_and_counts(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;

        let star = repo.star(created.id, bob).await.unwrap().expect("starred");
        assert_eq!(star.user_id, bob);
        assert_eq!(star.repository_id, created.id);

        let after = repo
            .get_by_id(created.id, Some(bob))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(after.stars, 1);
        assert!(after.user_star);

        // A repeat star is a no-op and must not double-count.
        assert!(repo.star(created.id, bob).await.unwrap().is_none());
        assert_eq!(
            repo.get_by_id(created.id, None)
                .await
                .unwrap()
                .unwrap()
                .stars,
            1
        );
    }

    #[sqlx::test]
    async fn unstar_decrements_and_reports(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;
        repo.star(created.id, bob).await.unwrap();

        assert!(repo.unstar(created.id, bob).await.unwrap());
        assert_eq!(
            repo.get_by_id(created.id, None)
                .await
                .unwrap()
                .unwrap()
                .stars,
            0
        );

        // Unstarring again reports false and the count floors at zero.
        assert!(!repo.unstar(created.id, bob).await.unwrap());
        assert_eq!(
            repo.get_by_id(created.id, None)
                .await
                .unwrap()
                .unwrap()
                .stars,
            0
        );
    }

    #[sqlx::test]
    async fn list_recent_stars_returns_users_newest_first(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let carol = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;
        insert_user(&pool, carol, "carol").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;

        let now = Utc::now();
        insert_star_at(&pool, bob, created.id, now - Duration::days(2)).await;
        insert_star_at(&pool, carol, created.id, now - Duration::days(1)).await;

        let stars = repo.list_recent_stars(created.id, 10).await.unwrap();
        assert_eq!(stars.len(), 2);
        // Most recent star first.
        assert_eq!(stars[0].0.name, "carol");
        assert_eq!(stars[1].0.name, "bob");

        // The limit truncates to the most recent.
        let limited = repo.list_recent_stars(created.id, 1).await.unwrap();
        assert_eq!(limited.len(), 1);
        assert_eq!(limited[0].0.name, "carol");
    }

    #[sqlx::test]
    async fn list_commit_filters_paginates(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;

        let now = Utc::now();
        insert_filter_at(&pool, created.id, "first", now - Duration::days(3)).await;
        insert_filter_at(&pool, created.id, "second", now - Duration::days(2)).await;
        insert_filter_at(&pool, created.id, "third", now - Duration::days(1)).await;

        let (page, cursor) = repo.list_commit_filters(created.id, None, 2).await.unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].name, "third");
        assert_eq!(page[1].name, "second");
        let cursor = cursor.expect("more rows remain");

        let (page, cursor) = repo
            .list_commit_filters(created.id, Some(cursor), 2)
            .await
            .unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].name, "first");
        assert!(cursor.is_none());
    }

    #[sqlx::test]
    async fn create_and_update_commit_filter(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;

        let filter = repo
            .create_commit_filter(
                created.id,
                "feat",
                Some(vec!["alice".to_string()]),
                None,
                Some(vec!["src/".to_string()]),
            )
            .await
            .unwrap();
        assert_eq!(filter.name, "feat");
        assert_eq!(filter.authors, Some(vec!["alice".to_string()]));
        assert_eq!(filter.tags, None);
        assert_eq!(filter.paths, Some(vec!["src/".to_string()]));

        let updated = repo
            .update_commit_filter(filter.id, "fix", None, Some(vec!["v1".to_string()]), None)
            .await
            .unwrap()
            .expect("updated");
        assert_eq!(updated.name, "fix");
        assert_eq!(updated.authors, None);
        assert_eq!(updated.tags, Some(vec!["v1".to_string()]));

        assert!(
            repo.update_commit_filter(Uuid::new_v4(), "x", None, None, None)
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn delete_commit_filter_reports(pool: PgPool) {
        let repo = RepositoryRepositoryImpl::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        let created = make_repo(&repo, "proj", alice, RepositoryVisibility::Public, None).await;
        let filter = repo
            .create_commit_filter(created.id, "feat", None, None, None)
            .await
            .unwrap();

        assert!(repo.delete_commit_filter(filter.id).await.unwrap());
        assert!(!repo.delete_commit_filter(filter.id).await.unwrap());
    }
}
