use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{GitHubInstallation, GitHubInstallationType},
};

#[async_trait]
pub trait GitHubRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        installation_id: i64,
        owner_id: Uuid,
        installation_type: GitHubInstallationType,
        github_login: &str,
    ) -> Result<GitHubInstallation, DatabaseError>;

    async fn get(
        &self,
        owner_id: Uuid,
        installation_id: i64,
    ) -> Result<Option<GitHubInstallation>, DatabaseError>;

    async fn delete_by_installation_id(&self, installation_id: i64) -> Result<(), DatabaseError>;

    async fn list_by_owner(
        &self,
        owner_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<GitHubInstallation>, Option<Cursor>), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct GitHubRepositoryImpl {
    pool: PgPool,
}

impl GitHubRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl GitHubRepository for GitHubRepositoryImpl {
    async fn create(
        &self,
        installation_id: i64,
        owner_id: Uuid,
        installation_type: GitHubInstallationType,
        github_login: &str,
    ) -> Result<GitHubInstallation, DatabaseError> {
        let installation = sqlx::query_as::<_, GitHubInstallation>(
            r#"
            INSERT INTO migration.github_installations (installation_id, owner_id, type, github_login)
            VALUES ($1, $2, $3, $4)
            RETURNING id, installation_id, owner_id, type, github_login, created_at
            "#,
        )
        .bind(installation_id)
        .bind(owner_id)
        .bind(installation_type)
        .bind(github_login)
        .fetch_one(&self.pool)
        .await?;

        Ok(installation)
    }

    async fn get(
        &self,
        owner_id: Uuid,
        installation_id: i64,
    ) -> Result<Option<GitHubInstallation>, DatabaseError> {
        let installation = sqlx::query_as::<_, GitHubInstallation>(
            r#"
            SELECT id, installation_id, owner_id, type, github_login, created_at
            FROM migration.github_installations
            WHERE owner_id = $1 AND installation_id = $2
            "#,
        )
        .bind(owner_id)
        .bind(installation_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(installation)
    }

    async fn delete_by_installation_id(&self, installation_id: i64) -> Result<(), DatabaseError> {
        sqlx::query("DELETE FROM migration.github_installations WHERE installation_id = $1")
            .bind(installation_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list_by_owner(
        &self,
        owner_id: Uuid,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<GitHubInstallation>, Option<Cursor>), DatabaseError> {
        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut installations = sqlx::query_as::<_, GitHubInstallation>(
            r#"
            SELECT id, installation_id, owner_id, type, github_login, created_at
            FROM migration.github_installations
            WHERE owner_id = $1
              AND ($2::timestamptz IS NULL OR (created_at, id) < ($2, $3))
            ORDER BY created_at DESC, id DESC
            LIMIT $4
            "#,
        )
        .bind(owner_id)
        .bind(cursor_created_at)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await?;

        let next_cursor = if installations.len() as i64 > limit {
            installations.pop();
            installations.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((installations, next_cursor))
    }
}
