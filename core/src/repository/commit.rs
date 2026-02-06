use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::Commit;

#[async_trait]
pub trait CommitRepository: Send + Sync + Clone + 'static {
    async fn create(
        &self,
        author_id: Uuid,
        repo_id: Uuid,
        sha: &str,
        message: &str,
    ) -> Result<Commit, Error>;
}

#[derive(Debug, Clone)]
pub struct CommitRepositoryImpl {
    pool: PgPool,
}

impl CommitRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommitRepository for CommitRepositoryImpl {
    async fn create(
        &self,
        author_id: Uuid,
        repo_id: Uuid,
        sha: &str,
        message: &str,
    ) -> Result<Commit, Error> {
        let commit = sqlx::query_as::<_, Commit>(
            r#"
            INSERT INTO commits (author_id, repo_id, sha, message)
            VALUES ($1, $2, $3, $4)
            RETURNING id, author_id, repo_id, sha, message, created_at
            "#,
        )
        .bind(author_id)
        .bind(repo_id)
        .bind(sha)
        .bind(message)
        .fetch_one(&self.pool)
        .await?;

        Ok(commit)
    }
}
