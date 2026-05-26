use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    dto::Cursor,
    error::DatabaseError,
    model::{Commit, CommitDiff},
};

const COMMIT_PROJECTION: &str = "
    c.id, c.repo_id, c.author_id, c.git_author_name, c.git_author_email,
    c.ref_name, c.sha, c.parent_sha, c.message, c.created_at,
    c.review_number, c.diff_position, c.diffs,
    au.name AS author_name,
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
    LEFT JOIN core.users au ON au.id = c.author_id
";

#[derive(FromRow)]
struct UserCommitRow {
    #[sqlx(flatten)]
    commit: Commit,
    viewer_has_access: bool,
}

#[async_trait]
pub trait CommitRepository: Send + Sync + Clone + 'static {
    async fn get_commit(&self, repo_id: Uuid, sha: &str) -> Result<Option<Commit>, DatabaseError>;

    async fn list_by_repository(
        &self,
        repo_id: Uuid,
        ref_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Commit>, Option<Cursor>), DatabaseError>;

    async fn list_by_user(
        &self,
        author_id: Uuid,
        viewer_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<(Commit, bool)>, Option<Cursor>), DatabaseError>;

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

    async fn list_by_repository(
        &self,
        repo_id: Uuid,
        ref_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<Commit>, Option<Cursor>), DatabaseError> {
        let query = format!(
            "SELECT {projection}
             FROM core.commits c
             {joins}
             WHERE c.repo_id = $1
               AND c.ref_name = $2
               AND c.created_at >= $3 AND c.created_at <= $4
               AND ($5::timestamptz IS NULL OR (c.created_at, c.id) < ($5, $6))
             ORDER BY c.created_at DESC, c.id DESC
             LIMIT $7",
            projection = COMMIT_PROJECTION,
            joins = COMMIT_JOINS,
        );

        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut commits = sqlx::query_as::<_, Commit>(&query)
            .bind(repo_id)
            .bind(ref_name)
            .bind(from)
            .bind(to)
            .bind(cursor_created_at)
            .bind(cursor_id)
            .bind(limit + 1)
            .fetch_all(&self.pool)
            .await?;

        let next_cursor = if commits.len() as i64 > limit {
            commits.pop();
            commits.last().map(|last| Cursor {
                created_at: last.created_at,
                id: last.id,
            })
        } else {
            None
        };

        Ok((commits, next_cursor))
    }

    async fn list_by_user(
        &self,
        author_id: Uuid,
        viewer_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        cursor: Option<Cursor>,
        limit: i64,
    ) -> Result<(Vec<(Commit, bool)>, Option<Cursor>), DatabaseError> {
        let query = format!(
            "WITH viewer_orgs AS (
                SELECT organization_id FROM core.organization_members WHERE user_id = $7
             )
             SELECT
                {projection},
                COALESCE(
                    r.visibility = 'public'
                    OR (r.owner_type = 'user' AND r.owner_id = $7)
                    OR (r.owner_type = 'organization'
                        AND r.owner_id IN (SELECT organization_id FROM viewer_orgs)),
                    false
                ) AS viewer_has_access
             FROM core.commits c
             {joins}
             WHERE c.author_id = $1
               AND c.created_at >= $2 AND c.created_at <= $3
               AND ($4::timestamptz IS NULL OR (c.created_at, c.id) < ($4, $5))
             ORDER BY c.created_at DESC, c.id DESC
             LIMIT $6",
            projection = COMMIT_PROJECTION,
            joins = COMMIT_JOINS,
        );

        let cursor_created_at = cursor.as_ref().map(|c| c.created_at);
        let cursor_id = cursor.as_ref().map(|c| c.id);

        let mut rows = sqlx::query_as::<_, UserCommitRow>(&query)
            .bind(author_id)
            .bind(from)
            .bind(to)
            .bind(cursor_created_at)
            .bind(cursor_id)
            .bind(limit + 1)
            .bind(viewer_id)
            .fetch_all(&self.pool)
            .await?;

        let next_cursor = if rows.len() as i64 > limit {
            rows.pop();
            rows.last().map(|last| Cursor {
                created_at: last.commit.created_at,
                id: last.commit.id,
            })
        } else {
            None
        };

        let data = rows
            .into_iter()
            .map(|r| (r.commit, r.viewer_has_access))
            .collect();

        Ok((data, next_cursor))
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
