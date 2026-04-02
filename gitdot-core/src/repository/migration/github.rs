use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
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

    async fn list_by_owner(&self, owner_id: Uuid)
    -> Result<Vec<GitHubInstallation>, DatabaseError>;
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

    async fn list_by_owner(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<GitHubInstallation>, DatabaseError> {
        let installations = sqlx::query_as::<_, GitHubInstallation>(
            r#"
            SELECT id, installation_id, owner_id, type, github_login, created_at
            FROM migration.github_installations
            WHERE owner_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(installations)
    }
}
