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
        github_login: &str,
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
        github_login: &str,
    ) -> Result<GitHubInstallation, Error> {
        sqlx::query_as::<_, GitHubInstallation>(
            r#"
            INSERT INTO github_installations (installation_id, owner_id, type, github_login)
            VALUES ($1, $2, $3, $4)
            RETURNING id, installation_id, owner_id, type, github_login, created_at
            "#,
        )
        .bind(installation_id)
        .bind(owner_id)
        .bind(installation_type)
        .bind(github_login)
        .fetch_one(&self.pool)
        .await
    }
}
