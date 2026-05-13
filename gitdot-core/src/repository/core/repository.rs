use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    dto::UserResponse,
    error::DatabaseError,
    model::{
        Repository, RepositoryOwnerType, RepositorySettings, RepositoryStar, RepositoryVisibility,
    },
};

#[async_trait]
pub trait RepositoryRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_name: &str,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
        description: Option<String>,
        readonly: bool,
    ) -> Result<Repository, DatabaseError>;

    async fn get(&self, owner: &str, repo: &str) -> Result<Option<Repository>, DatabaseError>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Repository>, DatabaseError>;

    async fn list_by_owner(&self, owner_name: &str) -> Result<Vec<Repository>, DatabaseError>;

    async fn delete(&self, id: Uuid) -> Result<(), DatabaseError>;

    async fn get_settings(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<RepositorySettings>, DatabaseError>;

    async fn update_settings(
        &self,
        owner: &str,
        repo: &str,
        settings: RepositorySettings,
    ) -> Result<Option<RepositorySettings>, DatabaseError>;

    async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, DatabaseError>;

    async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn is_starred(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError>;

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(UserResponse, DateTime<Utc>)>, DatabaseError>;
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
        owner_name: &str,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
        description: Option<String>,
        readonly: bool,
    ) -> Result<Repository, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            INSERT INTO core.repositories (name, owner_id, owner_name, owner_type, visibility, description, readonly)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, owner_id, owner_name, owner_type, visibility, description, stars, readonly, created_at
            "#,
        )
        .bind(name)
        .bind(owner_id)
        .bind(owner_name)
        .bind(owner_type)
        .bind(visibility)
        .bind(description)
        .bind(readonly)
        .fetch_one(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get(&self, owner: &str, repo: &str) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, visibility, description, stars, readonly, created_at
            FROM core.repositories
            WHERE owner_name = $1 AND name = $2
            "#,
        )
        .bind(owner)
        .bind(repo)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Repository>, DatabaseError> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, visibility, description, stars, readonly, created_at
            FROM core.repositories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn list_by_owner(&self, owner_name: &str) -> Result<Vec<Repository>, DatabaseError> {
        let repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, visibility, description, stars, readonly, created_at
            FROM core.repositories
            WHERE owner_name = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(owner_name)
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

    async fn get_settings(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<RepositorySettings>, DatabaseError> {
        let row = sqlx::query(
            "SELECT settings FROM core.repositories WHERE owner_name = $1 AND name = $2",
        )
        .bind(owner)
        .bind(repo)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else { return Ok(None) };
        let json: Option<serde_json::Value> = row.try_get("settings")?;
        Ok(Some(
            json.and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or(RepositorySettings {
                    commit_filters: None,
                }),
        ))
    }

    async fn update_settings(
        &self,
        owner: &str,
        repo: &str,
        settings: RepositorySettings,
    ) -> Result<Option<RepositorySettings>, DatabaseError> {
        let settings = serde_json::to_value(&settings).unwrap();
        let row = sqlx::query(
            "UPDATE core.repositories SET settings = COALESCE(settings, '{}'::jsonb) || $3::jsonb WHERE owner_name = $1 AND name = $2 RETURNING settings",
        )
        .bind(owner)
        .bind(repo)
        .bind(settings)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else { return Ok(None) };
        let json: Option<serde_json::Value> = row.try_get("settings")?;
        Ok(Some(
            json.and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or(RepositorySettings {
                    commit_filters: None,
                }),
        ))
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

    async fn is_starred(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError> {
        let row = sqlx::query(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM core.stars
                WHERE repository_id = $1 AND user_id = $2
            ) AS "starred"
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_get("starred")?)
    }

    async fn list_recent_stars(
        &self,
        repository_id: Uuid,
        limit: i64,
    ) -> Result<Vec<(UserResponse, DateTime<Utc>)>, DatabaseError> {
        let rows = sqlx::query(
            r#"
            SELECT u.id, u.name, u.email, u.created_at, u.display_name,
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

        rows.into_iter()
            .map(|row| {
                let user = UserResponse {
                    id: row.try_get("id")?,
                    name: row.try_get("name")?,
                    email: row.try_get("email")?,
                    created_at: row.try_get("created_at")?,
                    display_name: row.try_get("display_name")?,
                    location: None,
                    readme: None,
                    links: vec![],
                };
                let starred_at: DateTime<Utc> = row.try_get("starred_at")?;
                Ok((user, starred_at))
            })
            .collect()
    }
}
