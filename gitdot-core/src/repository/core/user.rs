use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rand::RngExt as _;
use sqlx::{FromRow, PgPool, Postgres};
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{AuthProvider, Repository, User, UserEmail},
    util::user::DEFAULT_USER_README,
};

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    name: String,
    provider: AuthProvider,
    created_at: DateTime<Utc>,
    display_name: Option<String>,
    location: Option<String>,
    readme: Option<String>,
    links: Vec<String>,
}

impl UserRow {
    fn into_user(self, emails: Vec<UserEmail>) -> User {
        User {
            id: self.id,
            name: self.name,
            provider: self.provider,
            created_at: self.created_at,
            display_name: self.display_name,
            location: self.location,
            readme: self.readme,
            links: self.links,
            emails,
        }
    }
}

async fn fetch_emails<'e, E>(executor: E, user_id: Uuid) -> Result<Vec<UserEmail>, DatabaseError>
where
    E: sqlx::Executor<'e, Database = Postgres>,
{
    let rows = sqlx::query_as::<_, UserEmail>(
        r#"
        SELECT id, user_id, email, is_primary, is_verified, created_at
        FROM core.user_emails
        WHERE user_id = $1
        ORDER BY is_primary DESC, created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(executor)
    .await?;

    Ok(rows)
}

#[derive(FromRow)]
struct StarredRepoRow {
    #[sqlx(flatten)]
    repository: Repository,
    starred_at: DateTime<Utc>,
    star_id: Uuid,
}

#[async_trait]
pub trait UserRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        email: &str,
        is_email_verified: bool,
        provider: AuthProvider,
    ) -> Result<User, DatabaseError>;

    async fn get(&self, user_name: &str) -> Result<Option<User>, DatabaseError>;

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
        display_name: Option<String>,
    ) -> Result<User, DatabaseError>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DatabaseError>;

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DatabaseError>;

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<(String, Uuid)>, DatabaseError>;

    async fn verify_email(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn is_name_taken(&self, name: &str) -> Result<bool, DatabaseError>;

    async fn list_emails(&self, user_id: Uuid) -> Result<Vec<UserEmail>, DatabaseError>;

    async fn create_email(&self, user_id: Uuid, email: &str) -> Result<UserEmail, DatabaseError>;

    async fn get_email(&self, email: &str) -> Result<Option<UserEmail>, DatabaseError>;

    async fn upsert_verified_emails(
        &self,
        user_id: Uuid,
        emails: &[String],
    ) -> Result<(), DatabaseError>;

    async fn list_starred_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> UserRepositoryImpl {
        UserRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(
        &self,
        email: &str,
        is_email_verified: bool,
        provider: AuthProvider,
    ) -> Result<User, DatabaseError> {
        let suffix: String = {
            let mut rng = rand::rng();
            let bytes: [u8; 4] = rng.random();
            hex::encode(bytes)
        };
        let name = format!("user_{suffix}");

        let mut tx = self.pool.begin().await?;

        let row = sqlx::query_as::<_, UserRow>(
            r#"
            INSERT INTO core.users (name, provider, readme)
            VALUES ($1, $2, $3)
            RETURNING id, name, provider, created_at, display_name, location, readme, links
            "#,
        )
        .bind(name)
        .bind(provider)
        .bind(DEFAULT_USER_README)
        .fetch_one(&mut *tx)
        .await?;

        let user_email = sqlx::query_as::<_, UserEmail>(
            r#"
            INSERT INTO core.user_emails (user_id, email, is_primary, is_verified, verified_at)
            VALUES ($1, $2, TRUE, $3, CASE WHEN $3 THEN NOW() ELSE NULL END)
            RETURNING id, user_id, email, is_primary, is_verified, created_at
            "#,
        )
        .bind(row.id)
        .bind(email)
        .bind(is_email_verified)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(row.into_user(vec![user_email]))
    }

    async fn get(&self, user_name: &str) -> Result<Option<User>, DatabaseError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, provider, created_at, display_name, location, readme, links
            FROM core.users
            WHERE name = $1
            "#,
        )
        .bind(user_name)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else { return Ok(None) };
        let emails = fetch_emails(&self.pool, row.id).await?;
        Ok(Some(row.into_user(emails)))
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
        let mut builder = sqlx::QueryBuilder::new("UPDATE core.users SET ");
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

        builder.push(" WHERE id = ").push_bind(id).push(
            " RETURNING id, name, provider, created_at, display_name, location, readme, links",
        );

        let row = builder
            .build_query_as::<UserRow>()
            .fetch_one(&self.pool)
            .await?;

        let emails = fetch_emails(&self.pool, row.id).await?;
        Ok(row.into_user(emails))
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DatabaseError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, provider, created_at, display_name, location, readme, links
            FROM core.users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else { return Ok(None) };
        let emails = fetch_emails(&self.pool, row.id).await?;
        Ok(Some(row.into_user(emails)))
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DatabaseError> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT u.id, u.name, u.provider, u.created_at, u.display_name, u.location, u.readme, u.links
            FROM core.users u
            JOIN core.user_emails ue ON ue.user_id = u.id AND ue.is_primary
            WHERE ue.email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else { return Ok(None) };
        let emails = fetch_emails(&self.pool, row.id).await?;
        Ok(Some(row.into_user(emails)))
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

    async fn list_emails(&self, user_id: Uuid) -> Result<Vec<UserEmail>, DatabaseError> {
        fetch_emails(&self.pool, user_id).await
    }

    async fn create_email(&self, user_id: Uuid, email: &str) -> Result<UserEmail, DatabaseError> {
        let row = sqlx::query_as::<_, UserEmail>(
            r#"
            INSERT INTO core.user_emails (user_id, email, is_primary, is_verified)
            VALUES ($1, $2, FALSE, FALSE)
            RETURNING id, user_id, email, is_primary, is_verified, created_at
            "#,
        )
        .bind(user_id)
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    async fn get_email(&self, email: &str) -> Result<Option<UserEmail>, DatabaseError> {
        let row = sqlx::query_as::<_, UserEmail>(
            r#"
            SELECT id, user_id, email, is_primary, is_verified, created_at
            FROM core.user_emails
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
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

    async fn list_starred_repositories(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut rows = sqlx::query_as::<_, StarredRepoRow>(
            r#"
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
}
