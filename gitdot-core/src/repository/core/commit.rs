use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{Commit, CommitDiff},
};

const COMMIT_PROJECTION: &str = "
    c.id, c.repo_id, c.author_id, c.git_author_name, c.git_author_email,
    c.ref_name, c.sha, c.parent_sha, c.message, c.created_at,
    c.review_number, c.diff_position, c.diffs,
    json_build_object(
        'id',         r.id,
        'owner_name', COALESCE(u.name, o.name),
        'name',       r.name,
        'visibility', r.visibility
    ) AS repository
";

const COMMIT_JOINS: &str = "
    JOIN core.repositories r ON c.repo_id = r.id
    LEFT JOIN core.users u
      ON r.owner_id = u.id AND r.owner_type = 'user'
    LEFT JOIN core.organizations o
      ON r.owner_id = o.id AND r.owner_type = 'organization'
";

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

        let query = format!(
            "SELECT {projection}
             FROM core.commits c
             {joins}
             WHERE c.repo_id = $1 AND c.sha_short = $2",
            projection = COMMIT_PROJECTION,
            joins = COMMIT_JOINS,
        );

        let commit = sqlx::query_as::<_, Commit>(&query)
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
        let query = format!(
            "SELECT {projection}
             FROM core.commits c
             {joins}
             WHERE c.repo_id = $1 AND c.created_at >= $2 AND c.created_at <= $3
             ORDER BY c.created_at DESC",
            projection = COMMIT_PROJECTION,
            joins = COMMIT_JOINS,
        );

        let commits = sqlx::query_as::<_, Commit>(&query)
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
        let query = format!(
            "SELECT {projection}
             FROM core.commits c
             {joins}
             WHERE c.author_id = $1 AND c.created_at >= $2 AND c.created_at <= $3
             ORDER BY c.created_at DESC",
            projection = COMMIT_PROJECTION,
            joins = COMMIT_JOINS,
        );

        let commits = sqlx::query_as::<_, Commit>(&query)
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

        let query = format!(
            r#"
            WITH inserted AS (
                INSERT INTO core.commits (
                    author_id, git_author_name, git_author_email, repo_id,
                    ref_name, sha, parent_sha, message, created_at, diffs,
                    review_number, diff_position
                )
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::text[], $3::text[], $4::uuid[],
                    $5::varchar[], $6::varchar[], $7::varchar[], $8::text[],
                    $9::timestamptz[], $10::jsonb[], $11::int[], $12::int[]
                )
                ON CONFLICT (repo_id, sha) DO NOTHING
                RETURNING *
            )
            SELECT {projection}
            FROM inserted c
            {joins}
            "#,
            projection = COMMIT_PROJECTION,
            joins = COMMIT_JOINS,
        );

        let rows = sqlx::query_as::<_, Commit>(&query)
            .bind(author_ids)
            .bind(git_author_names)
            .bind(git_author_emails)
            .bind(repo_ids)
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
