use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rand::RngExt as _;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{AuthProvider, Repository, User, UserEmail},
    util::user::DEFAULT_USER_README,
};

const USER_PROJECTION_QUERY: &str = r#"
SELECT
    u.id, u.name, u.provider, u.created_at, u.image_updated_at, u.display_name, u.location, u.readme, u.links,
    COALESCE(
        (SELECT json_agg(json_build_object(
            'id', e.id,
            'user_id', e.user_id,
            'email', e.email,
            'is_primary', e.is_primary,
            'is_verified', e.is_verified,
            'created_at', e.created_at
         ) ORDER BY e.is_primary DESC, e.created_at ASC)
         FROM core.user_emails e WHERE e.user_id = u.id),
        '[]'::json
    ) AS emails
"#;

/// sqlx data-access layer for the `core.users` and `core.user_emails` tables,
/// plus read-only joins against repositories, stars, commits, and organizations
/// for a user's repository listings.
#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    /// Inserts a new `core.users` row (with a random `user_<hex>` name and the
    /// default README) and its primary `core.user_emails` row in one CTE,
    /// returning the projected [`User`]. The email is marked verified (with
    /// `verified_at = NOW()`) only when `is_email_verified` is true.
    ///
    /// The data-modifying CTE inserts the email after the projection reads, so
    /// the returned [`User`] has an empty `emails`; re-fetch to see it.
    async fn create(
        &self,
        email: &str,
        is_email_verified: bool,
        provider: AuthProvider,
    ) -> Result<User, DatabaseError>;

    /// Returns the [`User`] (with aggregated emails) whose `name` matches, or
    /// `Ok(None)` if no such user exists.
    async fn get(&self, user_name: &str) -> Result<Option<User>, DatabaseError>;

    /// Updates the `core.users` row identified by `id`, dynamically setting only
    /// the `Some` fields (`name`, `location`, `readme`, `links`, `display_name`)
    /// and returning the projected [`User`] via `RETURNING`. Passing all `None`
    /// produces invalid SQL; callers must supply at least one field.
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
        display_name: Option<String>,
    ) -> Result<User, DatabaseError>;

    /// Returns the [`User`] (with aggregated emails) whose `id` matches, or
    /// `Ok(None)` if no such user exists.
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DatabaseError>;

    /// Sets `core.users.image_updated_at = now()` for the given `id`. A no-op if
    /// no row matches.
    async fn touch_image(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Returns the [`User`] whose primary `core.user_emails` row matches `email`,
    /// joining `users` to `user_emails` on `is_primary`. Non-primary addresses
    /// never match; `Ok(None)` if none.
    async fn get_by_primary_email(&self, email: &str) -> Result<Option<User>, DatabaseError>;

    /// Returns `(email, user_id)` pairs from `core.user_emails` for every input
    /// address that is verified (`is_verified = TRUE`). Short-circuits to an
    /// empty `Vec` when `emails` is empty; unmatched/unverified addresses are
    /// omitted.
    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<(String, Uuid)>, DatabaseError>;

    /// Marks the user's primary `core.user_emails` row verified, setting
    /// `is_verified = TRUE` and `verified_at` (preserving an existing
    /// `verified_at` via `COALESCE`). A no-op if no primary email matches.
    async fn verify_email(&self, id: Uuid) -> Result<(), DatabaseError>;

    /// Returns whether `name` is already used by a `core.users` or
    /// `core.organizations` row (namespaces share one slug space).
    async fn is_name_taken(&self, name: &str) -> Result<bool, DatabaseError>;

    /// Returns whether `email` is already verified by any user (a verified
    /// `core.user_emails` row exists for it).
    async fn is_email_taken(&self, email: &str) -> Result<bool, DatabaseError>;

    /// Lists all `core.user_emails` rows for `user_id`, ordered primary first
    /// then by `created_at ASC`.
    async fn list_emails(&self, user_id: Uuid) -> Result<Vec<UserEmail>, DatabaseError>;

    /// Bulk-inserts the given addresses as non-primary, verified
    /// (`verified_at = NOW()`) `core.user_emails` rows for `user_id` from an
    /// `UNNEST` of the input array. `ON CONFLICT DO NOTHING` skips any address
    /// already owned (by this user or, via the verified-email unique index, by
    /// another); other rows still insert. No-op when `emails` is empty.
    async fn upsert_verified_emails(
        &self,
        user_id: Uuid,
        emails: &[String],
    ) -> Result<(), DatabaseError>;

    /// Lists a user's own `core.repositories` (`owner_type = 'user'`,
    /// `owner_id = user_id`), cursor-paginated by `(created_at, id)` descending.
    /// Each row carries the owner's name, a `user_star` flag for `viewer_id`,
    /// and the user's own commit count plus last-commit time aggregated from
    /// `core.commits` where `author_id = user_id` (both `None` when there are no
    /// such commits). Private repos are visible only when `viewer_id` is the
    /// owner. Returns the page plus the next [`Cursor`] (`None` on the last
    /// page).
    #[allow(clippy::type_complexity)]
    async fn list_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<
        (
            Vec<(Repository, Option<i64>, Option<DateTime<Utc>>)>,
            Option<Cursor>,
        ),
        DatabaseError,
    >;

    /// Lists `core.repositories` that `user_id` has starred (joined through
    /// `core.stars`), cursor-paginated by the star's `(created_at, id)`
    /// descending. Visibility gates each repo to `viewer_id`: public, owned by
    /// the viewer, or owned by an org the viewer belongs to (via
    /// `core.organization_members`). Each row carries the owner's name and a
    /// `user_star` flag for `viewer_id`. Returns the page plus the next
    /// [`Cursor`] (`None` on the last page).
    async fn list_starred_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError>;

    /// Lists `core.repositories` that `user_id` has authored commits in on or
    /// after `since` (aggregated from `core.commits` where `author_id = user_id`
    /// and `created_at >= since`), with the per-repo commit count and last-commit
    /// time. Cursor-paginated by `(last_commit_at, repo id)` descending. The same
    /// `viewer_id` visibility gate as starred repos applies (public, viewer-owned,
    /// or viewer's-org-owned). Returns the page plus the next [`Cursor`] (`None`
    /// on the last page).
    async fn list_contributed_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        since: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<(Repository, i64, DateTime<Utc>)>, Option<Cursor>), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> PgUserRepository {
        PgUserRepository { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(
        &self,
        email: &str,
        is_email_verified: bool,
        provider: AuthProvider,
    ) -> Result<User, DatabaseError> {
        // The initial handle is random, so a collision against the unique idx_users_name
        // is astronomically unlikely but not impossible. Retry with a fresh handle before
        // surfacing the error.
        const MAX_ATTEMPTS: usize = 3;
        for attempt in 1..=MAX_ATTEMPTS {
            let name = {
                let mut rng = rand::rng();
                let bytes: [u8; 4] = rng.random();
                format!("user_{}", hex::encode(bytes))
            };

            let result = sqlx::query_as::<_, User>(
                r#"
            WITH new_user AS (
                INSERT INTO core.users (name, provider, readme)
                VALUES ($1, $2, $3)
                RETURNING id, name, provider, created_at, image_updated_at, display_name, location, readme, links
            ),
            new_email AS (
                INSERT INTO core.user_emails (user_id, email, is_primary, is_verified, verified_at)
                SELECT id, $4, TRUE, $5, CASE WHEN $5 THEN NOW() ELSE NULL END FROM new_user
            )
            SELECT
                u.id, u.name, u.provider, u.created_at, u.image_updated_at,
                u.display_name, u.location, u.readme, u.links,
                COALESCE(
                    (SELECT json_agg(json_build_object(
                        'id', e.id,
                        'user_id', e.user_id,
                        'email', e.email,
                        'is_primary', e.is_primary,
                        'is_verified', e.is_verified,
                        'created_at', e.created_at
                     ) ORDER BY e.is_primary DESC, e.created_at ASC)
                     FROM core.user_emails e WHERE e.user_id = u.id),
                    '[]'::json
                ) AS emails
            FROM new_user u
            "#,
            )
            .bind(name)
            .bind(provider.clone())
            .bind(DEFAULT_USER_README)
            .bind(email)
            .bind(is_email_verified)
            .fetch_one(&self.pool)
            .await;

            match result {
                Ok(user) => return Ok(user),
                Err(e) => {
                    let err = DatabaseError::from(e);
                    if err.is_unique_violation() && attempt < MAX_ATTEMPTS {
                        continue;
                    }
                    return Err(err);
                }
            }
        }

        unreachable!("create loop returns within MAX_ATTEMPTS")
    }

    async fn get(&self, user_name: &str) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as::<_, User>(&format!(
            "{USER_PROJECTION_QUERY} FROM core.users u WHERE u.name = $1"
        ))
        .bind(user_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
        display_name: Option<String>,
    ) -> Result<User, DatabaseError> {
        let mut builder = sqlx::QueryBuilder::new("WITH u AS (UPDATE core.users SET ");
        let mut sep = builder.separated(", ");

        if let Some(n) = name {
            sep.push("name = ").push_bind_unseparated(n);
        }
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
            .push(" WHERE id = ")
            .push_bind(id)
            .push(" RETURNING id, name, provider, created_at, image_updated_at, display_name, location, readme, links) ")
            .push(USER_PROJECTION_QUERY)
            .push(" FROM u");

        let user = builder
            .build_query_as::<User>()
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as::<_, User>(&format!(
            "{USER_PROJECTION_QUERY} FROM core.users u WHERE u.id = $1"
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn touch_image(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE core.users SET image_updated_at = now() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_by_primary_email(&self, email: &str) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as::<_, User>(&format!(
            r#"
            {USER_PROJECTION_QUERY}
            FROM core.users u
            JOIN core.user_emails ue ON ue.user_id = u.id AND ue.is_primary
            WHERE ue.email = $1
            "#
        ))
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<(String, Uuid)>, DatabaseError> {
        if emails.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, (String, Uuid)>(
            r#"
            SELECT email, user_id
            FROM core.user_emails
            WHERE email = ANY($1) AND is_verified = TRUE
            "#,
        )
        .bind(emails)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn verify_email(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE core.user_emails
            SET is_verified = TRUE, verified_at = COALESCE(verified_at, NOW())
            WHERE user_id = $1 AND is_primary
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn is_name_taken(&self, name: &str) -> Result<bool, DatabaseError> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM core.users WHERE name = $1
                UNION
                SELECT 1 FROM core.organizations WHERE name = $1
            )
            "#,
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn is_email_taken(&self, email: &str) -> Result<bool, DatabaseError> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM core.user_emails WHERE email = $1 AND is_verified
            )
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn list_emails(&self, user_id: Uuid) -> Result<Vec<UserEmail>, DatabaseError> {
        let rows = sqlx::query_as::<_, UserEmail>(
            r#"
            SELECT id, user_id, email, is_primary, is_verified, created_at
            FROM core.user_emails
            WHERE user_id = $1
            ORDER BY is_primary DESC, created_at ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn upsert_verified_emails(
        &self,
        user_id: Uuid,
        emails: &[String],
    ) -> Result<(), DatabaseError> {
        if emails.is_empty() {
            return Ok(());
        }

        // ON CONFLICT DO NOTHING (no target) skips on any unique violation —
        // both `(email) WHERE is_verified` (another user already owns it) and
        // `(user_id, email)` (this user already has it). Other emails in the
        // same batch still insert.
        sqlx::query(
            r#"
            INSERT INTO core.user_emails (user_id, email, is_primary, is_verified, verified_at)
            SELECT $1, e, FALSE, TRUE, NOW()
            FROM UNNEST($2::text[]) AS e
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(emails)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<
        (
            Vec<(Repository, Option<i64>, Option<DateTime<Utc>>)>,
            Option<Cursor>,
        ),
        DatabaseError,
    > {
        #[derive(FromRow)]
        struct UserRepoRow {
            #[sqlx(flatten)]
            repository: Repository,
            commit_count: Option<i64>,
            last_commit_at: Option<DateTime<Utc>>,
        }

        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut rows = sqlx::query_as::<_, UserRepoRow>(
            r#"
            WITH agg AS (
                SELECT c.repo_id, COUNT(*) AS commit_count, MAX(c.created_at) AS last_commit_at
                FROM core.commits c
                WHERE c.author_id = $1
                GROUP BY c.repo_id
            )
            SELECT r.id, r.name, r.owner_id, COALESCE(ru.name, ro.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars vs WHERE vs.repository_id = r.id AND vs.user_id = $5) AS user_star,
                   agg.commit_count, agg.last_commit_at
            FROM core.repositories r
            LEFT JOIN core.users ru
              ON r.owner_id = ru.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations ro
              ON r.owner_id = ro.id AND r.owner_type = 'organization'
            LEFT JOIN agg ON agg.repo_id = r.id
            WHERE r.owner_type = 'user' AND r.owner_id = $1
              AND (r.visibility = 'public' OR r.owner_id = $5)
              AND ($2::timestamptz IS NULL OR (r.created_at, r.id) < ($2, $3))
            ORDER BY r.created_at DESC, r.id DESC
            LIMIT $4
            "#,
        )
        .bind(user_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .bind(viewer_id)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if rows.len() as i64 > limit {
            rows.pop();
            rows.last().map(|last| Cursor {
                created_at: last.repository.created_at,
                id: last.repository.id,
            })
        } else {
            None
        };

        Ok((
            rows.into_iter()
                .map(|r| (r.repository, r.commit_count, r.last_commit_at))
                .collect(),
            next_cursor,
        ))
    }

    async fn list_starred_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError> {
        #[derive(FromRow)]
        struct StarredRepoRow {
            #[sqlx(flatten)]
            repository: Repository,
            starred_at: DateTime<Utc>,
            star_id: Uuid,
        }

        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut rows = sqlx::query_as::<_, StarredRepoRow>(
            r#"
            WITH viewer_orgs AS (
                SELECT organization_id FROM core.organization_members WHERE user_id = $5
            )
            SELECT r.id, r.name, r.owner_id, COALESCE(ru.name, ro.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars vs WHERE vs.repository_id = r.id AND vs.user_id = $5) AS user_star,
                   s.created_at AS starred_at, s.id AS star_id
            FROM core.stars s
            JOIN core.repositories r ON r.id = s.repository_id
            LEFT JOIN core.users ru
              ON r.owner_id = ru.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations ro
              ON r.owner_id = ro.id AND r.owner_type = 'organization'
            WHERE s.user_id = $1
              AND (
                  r.visibility = 'public'
                  OR (r.owner_type = 'user' AND r.owner_id = $5)
                  OR (r.owner_type = 'organization'
                      AND r.owner_id IN (SELECT organization_id FROM viewer_orgs))
              )
              AND ($2::timestamptz IS NULL OR (s.created_at, s.id) < ($2, $3))
            ORDER BY s.created_at DESC, s.id DESC
            LIMIT $4
            "#,
        )
        .bind(user_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .bind(viewer_id)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if rows.len() as i64 > limit {
            rows.pop();
            rows.last().map(|last| Cursor {
                created_at: last.starred_at,
                id: last.star_id,
            })
        } else {
            None
        };

        Ok((
            rows.into_iter().map(|r| r.repository).collect(),
            next_cursor,
        ))
    }

    async fn list_contributed_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        since: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<(Repository, i64, DateTime<Utc>)>, Option<Cursor>), DatabaseError> {
        #[derive(FromRow)]
        struct ContributedRepoRow {
            #[sqlx(flatten)]
            repository: Repository,
            commit_count: i64,
            last_commit_at: DateTime<Utc>,
        }

        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut rows = sqlx::query_as::<_, ContributedRepoRow>(
            r#"
            WITH viewer_orgs AS (
                SELECT organization_id FROM core.organization_members WHERE user_id = $5
            ),
            agg AS (
                SELECT c.repo_id, COUNT(*) AS commit_count, MAX(c.created_at) AS last_commit_at
                FROM core.commits c
                WHERE c.author_id = $1 AND c.created_at >= $6
                GROUP BY c.repo_id
            )
            SELECT r.id, r.name, r.owner_id, COALESCE(ru.name, ro.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
                   EXISTS(SELECT 1 FROM core.stars vs WHERE vs.repository_id = r.id AND vs.user_id = $5) AS user_star,
                   agg.commit_count, agg.last_commit_at
            FROM agg
            JOIN core.repositories r ON r.id = agg.repo_id
            LEFT JOIN core.users ru
              ON r.owner_id = ru.id AND r.owner_type = 'user'
            LEFT JOIN core.organizations ro
              ON r.owner_id = ro.id AND r.owner_type = 'organization'
            WHERE (
                r.visibility = 'public'
                OR (r.owner_type = 'user' AND r.owner_id = $5)
                OR (r.owner_type = 'organization'
                    AND r.owner_id IN (SELECT organization_id FROM viewer_orgs))
            )
              AND ($2::timestamptz IS NULL OR (agg.last_commit_at, r.id) < ($2, $3))
            ORDER BY agg.last_commit_at DESC, r.id DESC
            LIMIT $4
            "#,
        )
        .bind(user_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .bind(viewer_id)
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if rows.len() as i64 > limit {
            rows.pop();
            rows.last().map(|last| Cursor {
                created_at: last.last_commit_at,
                id: last.repository.id,
            })
        } else {
            None
        };

        Ok((
            rows.into_iter()
                .map(|r| (r.repository, r.commit_count, r.last_commit_at))
                .collect(),
            next_cursor,
        ))
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use chrono::{Duration, Months, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::{AuthProvider, PgUserRepository, UserRepository};
    use crate::repository::test_common::{
        insert_commit, insert_org, insert_secondary_email, insert_star, insert_user,
        insert_user_repo,
    };

    #[sqlx::test]
    async fn create_persists_user_with_primary_email(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());

        let user = repo
            .create("alice@example.com", true, AuthProvider::Email)
            .await
            .unwrap();
        assert!(user.name.starts_with("user_"));
        assert_eq!(user.provider, AuthProvider::Email);

        // `create` inserts the email in a data-modifying CTE whose rows aren't
        // visible to the same statement's projection, so its return value has an
        // empty `emails`; re-fetch to inspect the persisted primary email.
        let fetched = repo.get_by_id(user.id).await.unwrap().unwrap();
        let primary = fetched.primary_email().expect("primary email");
        assert_eq!(primary.email, "alice@example.com");
        assert!(primary.is_primary);
        assert!(primary.is_verified);

        // An unverified GitHub signup keeps the email unverified.
        let github = repo
            .create("bob@example.com", false, AuthProvider::GitHub)
            .await
            .unwrap();
        assert_eq!(github.provider, AuthProvider::GitHub);
        let github = repo.get_by_id(github.id).await.unwrap().unwrap();
        assert!(!github.primary_email().unwrap().is_verified);
    }

    #[sqlx::test]
    async fn get_and_get_by_id_round_trip(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let user = repo
            .create("a@x.com", true, AuthProvider::Email)
            .await
            .unwrap();

        let by_name = repo.get(&user.name).await.unwrap().expect("found by name");
        assert_eq!(by_name.id, user.id);
        let by_id = repo.get_by_id(user.id).await.unwrap().expect("found by id");
        assert_eq!(by_id.name, user.name);

        assert!(repo.get("no_such_user").await.unwrap().is_none());
        assert!(repo.get_by_id(Uuid::new_v4()).await.unwrap().is_none());
    }

    #[sqlx::test]
    async fn update_changes_only_provided_fields(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let user = repo
            .create("u@x.com", true, AuthProvider::Email)
            .await
            .unwrap();

        let updated = repo
            .update(
                user.id,
                Some("renamed".to_string()),
                Some("Earth".to_string()),
                Some("hello".to_string()),
                Some(vec!["https://example.com".to_string()]),
                Some("Display".to_string()),
            )
            .await
            .unwrap();
        assert_eq!(updated.name, "renamed");
        assert_eq!(updated.location.as_deref(), Some("Earth"));
        assert_eq!(updated.readme.as_deref(), Some("hello"));
        assert_eq!(updated.links, vec!["https://example.com".to_string()]);
        assert_eq!(updated.display_name.as_deref(), Some("Display"));

        // A partial update touches only the provided field.
        let again = repo
            .update(user.id, None, Some("Mars".to_string()), None, None, None)
            .await
            .unwrap();
        assert_eq!(again.name, "renamed");
        assert_eq!(again.location.as_deref(), Some("Mars"));
        assert_eq!(again.readme.as_deref(), Some("hello"));
    }

    #[sqlx::test]
    async fn touch_image_advances_timestamp(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let id = Uuid::new_v4();
        // Seed with an old image timestamp so the touch is observable.
        sqlx::query(
            "INSERT INTO core.users (id, name, image_updated_at)
             VALUES ($1, $2, NOW() - INTERVAL '1 day')",
        )
        .bind(id)
        .bind("imguser")
        .execute(&pool)
        .await
        .unwrap();

        let before = repo.get_by_id(id).await.unwrap().unwrap().image_updated_at;
        repo.touch_image(id).await.unwrap();
        let after = repo.get_by_id(id).await.unwrap().unwrap().image_updated_at;
        assert!(after > before, "expected {after} > {before}");
    }

    #[sqlx::test]
    async fn get_by_email_matches_primary_only(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let user = repo
            .create("primary@x.com", true, AuthProvider::Email)
            .await
            .unwrap();
        insert_secondary_email(&pool, user.id, "secondary@x.com").await;

        let found = repo
            .get_by_primary_email("primary@x.com")
            .await
            .unwrap()
            .expect("found by primary email");
        assert_eq!(found.id, user.id);

        // A non-primary email must not resolve a user here.
        assert!(
            repo.get_by_primary_email("secondary@x.com")
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            repo.get_by_primary_email("missing@x.com")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test]
    async fn get_by_emails_returns_verified_only(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let verified = repo
            .create("verified@x.com", true, AuthProvider::Email)
            .await
            .unwrap();
        repo.create("unverified@x.com", false, AuthProvider::Email)
            .await
            .unwrap();

        let rows = repo
            .get_by_emails(&[
                "verified@x.com".to_string(),
                "unverified@x.com".to_string(),
                "missing@x.com".to_string(),
            ])
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "verified@x.com");
        assert_eq!(rows[0].1, verified.id);

        assert!(repo.get_by_emails(&[]).await.unwrap().is_empty());
    }

    #[sqlx::test]
    async fn verify_email_marks_primary_verified(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let user = repo
            .create("v@x.com", false, AuthProvider::Email)
            .await
            .unwrap();
        let initial = repo.get_by_id(user.id).await.unwrap().unwrap();
        assert!(!initial.primary_email().unwrap().is_verified);

        repo.verify_email(user.id).await.unwrap();

        let refreshed = repo.get_by_id(user.id).await.unwrap().unwrap();
        assert!(refreshed.primary_email().unwrap().is_verified);
    }

    #[sqlx::test]
    async fn is_name_taken_checks_users_and_orgs(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let user = repo
            .create("n@x.com", true, AuthProvider::Email)
            .await
            .unwrap();
        insert_org(&pool, Uuid::new_v4(), "acme").await;

        assert!(repo.is_name_taken(&user.name).await.unwrap());
        assert!(repo.is_name_taken("acme").await.unwrap());
        assert!(!repo.is_name_taken("totally_unused_name").await.unwrap());
    }

    #[sqlx::test]
    async fn list_emails_orders_primary_first(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let user = repo
            .create("primary@x.com", true, AuthProvider::Email)
            .await
            .unwrap();
        insert_secondary_email(&pool, user.id, "second@x.com").await;

        let emails = repo.list_emails(user.id).await.unwrap();
        assert_eq!(emails.len(), 2);
        assert!(emails[0].is_primary);
        assert_eq!(emails[0].email, "primary@x.com");
        assert!(!emails[1].is_primary);
        assert_eq!(emails[1].email, "second@x.com");
    }

    #[sqlx::test]
    async fn upsert_verified_emails_inserts_and_skips_conflicts(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let a = repo
            .create("a@x.com", true, AuthProvider::Email)
            .await
            .unwrap();
        let b = repo
            .create("b@x.com", true, AuthProvider::Email)
            .await
            .unwrap();

        repo.upsert_verified_emails(a.id, &["x@a.com".to_string(), "y@a.com".to_string()])
            .await
            .unwrap();
        let emails = repo.list_emails(a.id).await.unwrap();
        assert_eq!(emails.len(), 3); // primary + two new
        assert!(
            emails
                .iter()
                .filter(|e| !e.is_primary)
                .all(|e| e.is_verified)
        );

        // A duplicate for A is skipped, and a verified email already owned by A
        // can't be claimed by B — only the genuinely new address lands.
        repo.upsert_verified_emails(b.id, &["x@a.com".to_string(), "z@b.com".to_string()])
            .await
            .unwrap();
        let b_emails = repo.list_emails(b.id).await.unwrap();
        assert_eq!(b_emails.len(), 2); // primary + z@b.com only
        assert!(b_emails.iter().any(|e| e.email == "z@b.com"));
        assert!(!b_emails.iter().any(|e| e.email == "x@a.com"));

        // Empty input is a no-op.
        repo.upsert_verified_emails(a.id, &[]).await.unwrap();
        assert_eq!(repo.list_emails(a.id).await.unwrap().len(), 3);
    }

    #[sqlx::test]
    async fn list_repositories_filters_visibility_and_counts_own_commits(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;

        let pub_repo = Uuid::new_v4();
        let priv_repo = Uuid::new_v4();
        insert_user_repo(&pool, pub_repo, "pub", alice, "public").await;
        insert_user_repo(&pool, priv_repo, "priv", alice, "private").await;

        let now = Utc::now();
        // Alice authored two commits in her public repo, ...
        insert_commit(
            &pool,
            pub_repo,
            alice,
            &"a".repeat(40),
            now - Duration::days(2),
        )
        .await;
        insert_commit(
            &pool,
            pub_repo,
            alice,
            &"b".repeat(40),
            now - Duration::days(1),
        )
        .await;
        // ... bob's commit in the same repo must not be counted, ...
        insert_commit(&pool, pub_repo, bob, &"c".repeat(40), now).await;
        // ... and alice has no commits in her private repo.

        // Owner sees both repos; stats reflect only alice's own commits.
        let (rows, _) = repo
            .list_repositories(alice, Some(alice), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);

        let public = rows.iter().find(|(r, _, _)| r.name == "pub").unwrap();
        assert_eq!(public.1, Some(2));
        let last = public.2.expect("public repo should have a last_commit_at");
        // bob's later commit is excluded, so the max is alice's day-1 commit.
        assert!(last < now);

        let private = rows.iter().find(|(r, _, _)| r.name == "priv").unwrap();
        assert_eq!(private.1, None);
        assert_eq!(private.2, None);

        // A different viewer only sees the public repo.
        let (rows, _) = repo
            .list_repositories(alice, Some(bob), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0.name, "pub");

        // Anonymous viewer only sees the public repo.
        let (rows, _) = repo.list_repositories(alice, None, None, 20).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0.name, "pub");
    }

    #[sqlx::test]
    async fn starred_repos_respect_viewer_visibility(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;
        insert_user(&pool, bob, "bob").await;

        let public_repo = Uuid::new_v4();
        let private_repo = Uuid::new_v4();
        insert_user_repo(&pool, public_repo, "pub", alice, "public").await;
        insert_user_repo(&pool, private_repo, "priv", alice, "private").await;
        insert_star(&pool, alice, public_repo).await;
        insert_star(&pool, alice, private_repo).await;

        // Owner sees both their public and private starred repos.
        let (repos, _) = repo
            .list_starred_repositories(alice, Some(alice), None, 20)
            .await
            .unwrap();
        assert_eq!(repos.len(), 2);

        // A different viewer only sees the public one.
        let (repos, _) = repo
            .list_starred_repositories(alice, Some(bob), None, 20)
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "pub");

        // Anonymous viewer only sees the public one.
        let (repos, _) = repo
            .list_starred_repositories(alice, None, None, 20)
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "pub");
    }

    #[sqlx::test]
    async fn contributed_repos_respect_from_window(pool: PgPool) {
        let repo = PgUserRepository::new(pool.clone());
        let alice = Uuid::new_v4();
        insert_user(&pool, alice, "alice").await;

        let recent_repo = Uuid::new_v4();
        let old_repo = Uuid::new_v4();
        insert_user_repo(&pool, recent_repo, "recent", alice, "public").await;
        insert_user_repo(&pool, old_repo, "old", alice, "public").await;

        let now = Utc::now();
        insert_commit(
            &pool,
            recent_repo,
            alice,
            &"a".repeat(40),
            now - Duration::days(1),
        )
        .await;
        insert_commit(
            &pool,
            old_repo,
            alice,
            &"b".repeat(40),
            now - Duration::days(800),
        )
        .await;

        // 12-month window excludes the ~2-year-old contribution.
        let (rows, _) = repo
            .list_contributed_repositories(alice, Some(alice), now - Months::new(12), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        let (r, count, _last) = &rows[0];
        assert_eq!(r.name, "recent");
        assert_eq!(*count, 1);

        // A wider window includes both.
        let (rows, _) = repo
            .list_contributed_repositories(alice, Some(alice), now - Months::new(60), None, 20)
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);
    }
}
