use async_trait::async_trait;
use sqlx::{Error, PgPool, Row};
use uuid::Uuid;

use crate::model::{Repository, RepositoryOwnerType, RepositorySettings, RepositoryVisibility};

#[async_trait]
pub trait RepositoryRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        name: &str,
        owner_id: Uuid,
        owner_name: &str,
        owner_type: &RepositoryOwnerType,
        visibility: &RepositoryVisibility,
    ) -> Result<Repository, Error>;

    async fn get(&self, owner: &str, repo: &str) -> Result<Option<Repository>, Error>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Repository>, Error>;

    async fn list_by_owner(&self, owner_name: &str) -> Result<Vec<Repository>, Error>;

    async fn delete(&self, id: Uuid) -> Result<(), Error>;

    async fn get_settings(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<RepositorySettings>, Error>;

    async fn update_settings(
        &self,
        owner: &str,
        repo: &str,
        settings: RepositorySettings,
    ) -> Result<Option<RepositorySettings>, Error>;
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
    ) -> Result<Repository, Error> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            INSERT INTO core.repositories (name, owner_id, owner_name, owner_type, visibility)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, owner_id, owner_name, owner_type, visibility, created_at
            "#,
        )
        .bind(name)
        .bind(owner_id)
        .bind(owner_name)
        .bind(owner_type)
        .bind(visibility)
        .fetch_one(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn get(&self, owner: &str, repo: &str) -> Result<Option<Repository>, Error> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, visibility, created_at
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

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Repository>, Error> {
        let repository = sqlx::query_as::<_, Repository>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, visibility, created_at
            FROM core.repositories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(repository)
    }

    async fn list_by_owner(&self, owner_name: &str) -> Result<Vec<Repository>, Error> {
        let repositories = sqlx::query_as::<_, Repository>(
            r#"
            SELECT id, name, owner_id, owner_name, owner_type, visibility, created_at
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

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
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
    ) -> Result<Option<RepositorySettings>, Error> {
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
    ) -> Result<Option<RepositorySettings>, Error> {
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
}
