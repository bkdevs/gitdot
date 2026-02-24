use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{GitHubInstallation, GitHubInstallationType};

#[async_trait]
pub trait GitHubRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        installation_id: i64,
        owner_id: Uuid,
        installation_type: GitHubInstallationType,
    ) -> Result<GitHubInstallation, Error>;
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

#[async_trait]
impl GitHubRepository for GitHubRepositoryImpl {
    async fn create(
        &self,
        installation_id: i64,
        owner_id: Uuid,
        installation_type: GitHubInstallationType,
    ) -> Result<GitHubInstallation, Error> {
        sqlx::query_as::<_, GitHubInstallation>(
            r#"
            INSERT INTO github_installations (installation_id, owner_id, type)
            VALUES ($1, $2, $3)
            RETURNING id, installation_id, owner_id, type, created_at
            "#,
        )
        .bind(installation_id)
        .bind(owner_id)
        .bind(installation_type)
        .fetch_one(&self.pool)
        .await
    }
}
