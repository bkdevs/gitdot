use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Commit, CommitDiff};

#[async_trait]
pub trait CommitRepository: Send + Sync + Clone + 'static {
    async fn create_bulk(
        &self,
        author_ids: &[Option<Uuid>],
        git_author_names: &[String],
        git_author_emails: &[String],
        repo_ids: &[Uuid],
        ref_names: &[String],
        shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
        diffs: &[Vec<CommitDiff>],
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

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl CommitRepository for CommitRepositoryImpl {
    async fn create_bulk(
        &self,
        author_ids: &[Option<Uuid>],
        git_author_names: &[String],
        git_author_emails: &[String],
        repo_ids: &[Uuid],
        ref_names: &[String],
        shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
        diffs: &[Vec<CommitDiff>],
    ) -> Result<Vec<Commit>, Error> {
        if shas.is_empty() {
            return Ok(Vec::new());
        }

        let diffs_json: Vec<serde_json::Value> = diffs
            .iter()
            .map(|d| serde_json::to_value(d).unwrap_or(serde_json::Value::Array(vec![])))
            .collect();

        let rows = sqlx::query_as::<_, Commit>(
            r#"
            INSERT INTO commits (author_id, git_author_name, git_author_email, repo_id, ref_name, sha, message, created_at, diffs)
            SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::text[], $4::uuid[], $5::varchar[], $6::varchar[], $7::text[], $8::timestamptz[], $9::jsonb[])
            RETURNING *
            "#,
        )
        .bind(author_ids)
        .bind(git_author_names)
        .bind(git_author_emails)
        .bind(repo_ids)
        .bind(ref_names)
        .bind(shas)
        .bind(messages)
        .bind(created_ats)
        .bind(diffs_json)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
