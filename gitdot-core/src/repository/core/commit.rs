use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{Commit, CommitDiff},
};

#[async_trait]
pub trait CommitRepository: Send + Sync + Clone + 'static {
    async fn get_commit(&self, repo_id: Uuid, sha: &str) -> Result<Option<Commit>, DatabaseError>;

    async fn get_commits(
        &self,
        repo_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Commit>, DatabaseError>;

    async fn list_by_user(
        &self,
        author_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Commit>, DatabaseError>;

    async fn create_bulk(
        &self,
        author_ids: &[Option<Uuid>],
        git_author_names: &[String],
        git_author_emails: &[String],
        repo_ids: &[Uuid],
        owner_names: &[String],
        repo_names: &[String],
        ref_names: &[String],
        shas: &[String],
        parent_shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
        diffs: &[Vec<CommitDiff>],
        review_numbers: &[Option<i32>],
        diff_positions: &[Option<i32>],
    ) -> Result<Vec<Commit>, DatabaseError>;
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
    async fn get_commit(&self, repo_id: Uuid, sha: &str) -> Result<Option<Commit>, DatabaseError> {
        let short = if sha.len() >= 7 { &sha[..7] } else { sha };

        let commit = sqlx::query_as::<_, Commit>(
            r#"
            SELECT * FROM core.commits
            WHERE repo_id = $1 AND sha_short = $2
            "#,
        )
        .bind(repo_id)
        .bind(short)
        .fetch_optional(&self.pool)
        .await?;

        Ok(commit)
    }

    async fn get_commits(
        &self,
        repo_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Commit>, DatabaseError> {
        let commits = sqlx::query_as::<_, Commit>(
            r#"
            SELECT * FROM core.commits
            WHERE repo_id = $1 AND created_at >= $2 AND created_at <= $3
            ORDER BY created_at DESC
            "#,
        )
        .bind(repo_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(commits)
    }

    async fn list_by_user(
        &self,
        author_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Commit>, DatabaseError> {
        let commits = sqlx::query_as::<_, Commit>(
            r#"
            SELECT * FROM core.commits
            WHERE author_id = $1 AND created_at >= $2 AND created_at <= $3
            ORDER BY created_at DESC
            "#,
        )
        .bind(author_id)
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(commits)
    }

    async fn create_bulk(
        &self,
        author_ids: &[Option<Uuid>],
        git_author_names: &[String],
        git_author_emails: &[String],
        repo_ids: &[Uuid],
        owner_names: &[String],
        repo_names: &[String],
        ref_names: &[String],
        shas: &[String],
        parent_shas: &[String],
        messages: &[String],
        created_ats: &[DateTime<Utc>],
        diffs: &[Vec<CommitDiff>],
        review_numbers: &[Option<i32>],
        diff_positions: &[Option<i32>],
    ) -> Result<Vec<Commit>, DatabaseError> {
        if shas.is_empty() {
            return Ok(Vec::new());
        }

        let diffs_json: Vec<serde_json::Value> = diffs
            .iter()
            .map(|d| serde_json::to_value(d).unwrap_or(serde_json::Value::Array(vec![])))
            .collect();

        let rows = sqlx::query_as::<_, Commit>(
            r#"
            INSERT INTO core.commits (author_id, git_author_name, git_author_email, repo_id, owner_name, repo_name, ref_name, sha, parent_sha, message, created_at, diffs, review_number, diff_position)
            SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::text[], $4::uuid[], $5::text[], $6::text[], $7::varchar[], $8::varchar[], $9::varchar[], $10::text[], $11::timestamptz[], $12::jsonb[], $13::int[], $14::int[])
            ON CONFLICT (repo_id, sha) DO NOTHING
            RETURNING *
            "#,
        )
        .bind(author_ids)
        .bind(git_author_names)
        .bind(git_author_emails)
        .bind(repo_ids)
        .bind(owner_names)
        .bind(repo_names)
        .bind(ref_names)
        .bind(shas)
        .bind(parent_shas)
        .bind(messages)
        .bind(created_ats)
        .bind(diffs_json)
        .bind(review_numbers)
        .bind(diff_positions)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
