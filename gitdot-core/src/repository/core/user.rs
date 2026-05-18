use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rand::RngExt as _;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{AuthProvider, Repository, User},
};

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

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<User>, DatabaseError>;

    async fn verify_email(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn is_name_taken(&self, name: &str) -> Result<bool, DatabaseError>;

    async fn is_email_taken(&self, email: &str) -> Result<bool, DatabaseError>;

    async fn list_starred_repositories(
        &self,
        user_id: Uuid,
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
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO core.users (email, name, is_email_verified, provider)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(name)
        .bind(is_email_verified)
        .bind(provider)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get(&self, user_name: &str) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, location, readme, links, display_name
            FROM core.users
            WHERE name = $1
            "#,
        )
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
            " RETURNING id, email, name, is_email_verified, provider, created_at, location, readme, links, display_name",
        );

        Ok(builder
            .build_query_as::<User>()
            .fetch_one(&self.pool)
            .await?)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, location, readme, links, display_name
            FROM core.users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, location, readme, links, display_name
            FROM core.users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<User>, DatabaseError> {
        if emails.is_empty() {
            return Ok(Vec::new());
        }

        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, name, is_email_verified, provider, created_at, location, readme, links, display_name
            FROM core.users
            WHERE email = ANY($1)
            "#,
        )
        .bind(emails)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn verify_email(&self, id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE core.users SET is_email_verified = true WHERE id = $1
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
            SELECT EXISTS(SELECT 1 FROM core.users WHERE email = $1)
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    async fn list_starred_repositories(
        &self,
        user_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Repository>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut rows = sqlx::query_as::<_, StarredRepoRow>(
            r#"
            SELECT r.id, r.name, r.owner_id, COALESCE(ru.name, ro.name) AS owner_name,
                   r.owner_type, r.visibility, r.description, r.stars, r.readonly, r.created_at,
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
