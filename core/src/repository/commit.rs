use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::commit::Commit;

#[async_trait]
pub trait CommitRepository: Send + Sync + Clone + 'static {
    async fn create_bulk(
        &self,
        author_ids: &[Uuid],
        repo_ids: &[Uuid],
        ref_names: &[String],
        shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
    ) -> Result<Vec<Commit>, Error>;
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
    async fn create_bulk(
        &self,
        author_ids: &[Uuid],
        repo_ids: &[Uuid],
        ref_names: &[String],
        shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
    ) -> Result<Vec<Commit>, Error> {
        if author_ids.is_empty() {
            return Ok(Vec::new());
        }

        let rows = sqlx::query_as::<_, Commit>(
            r#"
            INSERT INTO commits (author_id, repo_id, ref_name, sha, message, created_at)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::varchar[], $4::varchar[], $5::text[], $6::timestamptz[])
            RETURNING id, author_id, repo_id, ref_name, sha, message, created_at
            "#,
        )
        .bind(author_ids)
        .bind(repo_ids)
        .bind(ref_names)
        .bind(shas)
        .bind(messages)
        .bind(created_ats)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
