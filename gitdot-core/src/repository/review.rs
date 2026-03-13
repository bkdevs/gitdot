use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Diff, Review, Reviewer, Revision};

const REVIEW_DETAILS_QUERY: &str = r#"
SELECT
    r.id,
    r.repository_id,
    r.number,
    r.author_id,
    r.title,
    r.description,
    r.target_branch,
    r.status,
    r.created_at,
    r.updated_at,

    -- Author
    (SELECT json_build_object(
        'id', u.id, 'name', u.name, 'email', u.email, 'created_at', u.created_at
    ) FROM users u WHERE u.id = r.author_id) AS author,

    -- Diffs with nested revisions
    COALESCE(
        (
            SELECT json_agg(
                json_build_object(
                    'id', d.id,
                    'review_id', d.review_id,
                    'position', d.position,
                    'title', d.title,
                    'description', d.description,
                    'status', d.status,
                    'created_at', d.created_at,
                    'updated_at', d.updated_at,
                    'revisions', COALESCE(
                        (
                            SELECT json_agg(
                                json_build_object(
                                    'id', rev.id,
                                    'diff_id', rev.diff_id,
                                    'number', rev.number,
                                    'commit_hash', rev.commit_hash,
                                    'created_at', rev.created_at
                                ) ORDER BY rev.number DESC
                            )
                            FROM revisions rev
                            WHERE rev.diff_id = d.id
                        ),
                        '[]'::json
                    )
                ) ORDER BY d.position ASC
            )
            FROM diffs d
            WHERE d.review_id = r.id
        ),
        '[]'::json
    ) AS diffs,

    -- Reviewers
    COALESCE(
        (
            SELECT json_agg(
                json_build_object(
                    'id', rv.id,
                    'review_id', rv.review_id,
                    'reviewer_id', rv.reviewer_id,
                    'status', rv.status,
                    'created_at', rv.created_at,
                    'user', (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'created_at', u.created_at)
                             FROM users u WHERE u.id = rv.reviewer_id)
                )
            )
            FROM reviewers rv
            WHERE rv.review_id = r.id
        ),
        '[]'::json
    ) AS reviewers,

    -- Comments
    COALESCE(
        (
            SELECT json_agg(
                json_build_object(
                    'id', c.id,
                    'review_id', c.review_id,
                    'diff_id', c.diff_id,
                    'revision_id', c.revision_id,
                    'author_id', c.author_id,
                    'parent_id', c.parent_id,
                    'body', c.body,
                    'file_path', c.file_path,
                    'line_number', c.line_number,
                    'side', c.side,
                    'resolved', c.resolved,
                    'created_at', c.created_at,
                    'updated_at', c.updated_at,
                    'author', (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'created_at', u.created_at)
                               FROM users u WHERE u.id = c.author_id)
                ) ORDER BY c.created_at ASC
            )
            FROM review_comments c
            WHERE c.review_id = r.id
        ),
        '[]'::json
    ) AS comments
FROM reviews r
"#;

#[async_trait]
pub trait ReviewRepository: Send + Sync + Clone + 'static {
    async fn get_review(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Option<Review>, Error>;

    async fn get_reviews(&self, owner: &str, repo: &str) -> Result<Vec<Review>, Error>;

    async fn create_review(
        &self,
        repository_id: Uuid,
        author_id: Uuid,
        target_branch: &str,
    ) -> Result<Review, Error>;

    async fn create_diff(
        &self,
        review_id: Uuid,
        position: i32,
        title: &str,
        description: &str,
    ) -> Result<Diff, Error>;

    async fn create_revision(
        &self,
        diff_id: Uuid,
        number: i32,
        commit_hash: &str,
    ) -> Result<Revision, Error>;

    async fn add_reviewer(
        &self,
        review_id: Uuid,
        reviewer_id: Uuid,
    ) -> Result<Option<Reviewer>, Error>;

    async fn remove_reviewer(&self, review_id: Uuid, reviewer_id: Uuid) -> Result<bool, Error>;
}

#[derive(Debug, Clone)]
pub struct ReviewRepositoryImpl {
    pool: PgPool,
}

impl ReviewRepositoryImpl {
    pub fn new(pool: PgPool) -> ReviewRepositoryImpl {
        ReviewRepositoryImpl { pool }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl ReviewRepository for ReviewRepositoryImpl {
    async fn get_review(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Option<Review>, Error> {
        let query = format!(
            "{} JOIN repositories repo ON r.repository_id = repo.id WHERE repo.owner_name = $1 AND repo.name = $2 AND r.number = $3",
            REVIEW_DETAILS_QUERY
        );

        sqlx::query_as::<_, Review>(&query)
            .bind(owner)
            .bind(repo)
            .bind(number)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_reviews(&self, owner: &str, repo: &str) -> Result<Vec<Review>, Error> {
        sqlx::query_as::<_, Review>(
            r#"
            SELECT
                r.id, r.repository_id, r.number, r.author_id, r.title, r.description,
                r.target_branch, r.status, r.created_at, r.updated_at,
                NULL AS author, NULL AS diffs, NULL AS reviewers, NULL AS comments
            FROM reviews r
            JOIN repositories repo ON r.repository_id = repo.id
            WHERE repo.owner_name = $1 AND repo.name = $2
            ORDER BY r.created_at DESC
            "#,
        )
        .bind(owner)
        .bind(repo)
        .fetch_all(&self.pool)
        .await
    }

    async fn create_review(
        &self,
        repository_id: Uuid,
        author_id: Uuid,
        target_branch: &str,
    ) -> Result<Review, Error> {
        sqlx::query_as::<_, Review>(
            r#"
            WITH next_number AS (
                SELECT COALESCE(MAX(number), 0) + 1 AS number
                FROM reviews
                WHERE repository_id = $1
            )
            INSERT INTO reviews (repository_id, number, author_id, title, description, target_branch)
            SELECT $1, next_number.number, $2, '', '', $3
            FROM next_number
            RETURNING
                id, repository_id, number, author_id, title, description,
                target_branch, status, created_at, updated_at,
                NULL AS author, NULL AS diffs, NULL AS reviewers, NULL AS comments
            "#,
        )
        .bind(repository_id)
        .bind(author_id)
        .bind(target_branch)
        .fetch_one(&self.pool)
        .await
    }

    async fn create_diff(
        &self,
        review_id: Uuid,
        position: i32,
        title: &str,
        description: &str,
    ) -> Result<Diff, Error> {
        sqlx::query_as::<_, Diff>(
            r#"
            INSERT INTO diffs (review_id, position, title, description)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id, review_id, position, title, description,
                status, created_at, updated_at,
                NULL AS revisions
            "#,
        )
        .bind(review_id)
        .bind(position)
        .bind(title)
        .bind(description)
        .fetch_one(&self.pool)
        .await
    }

    async fn create_revision(
        &self,
        diff_id: Uuid,
        number: i32,
        commit_hash: &str,
    ) -> Result<Revision, Error> {
        sqlx::query_as::<_, Revision>(
            r#"
            INSERT INTO revisions (diff_id, number, commit_hash)
            VALUES ($1, $2, $3)
            RETURNING id, diff_id, number, commit_hash, created_at
            "#,
        )
        .bind(diff_id)
        .bind(number)
        .bind(commit_hash)
        .fetch_one(&self.pool)
        .await
    }

    async fn add_reviewer(
        &self,
        review_id: Uuid,
        reviewer_id: Uuid,
    ) -> Result<Option<Reviewer>, Error> {
        sqlx::query_as::<_, Reviewer>(
            r#"
            INSERT INTO reviewers (review_id, reviewer_id, status)
            VALUES ($1, $2, 'pending')
            ON CONFLICT (review_id, reviewer_id) DO NOTHING
            RETURNING
                id, review_id, reviewer_id, status, created_at,
                (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'created_at', u.created_at)
                 FROM users u WHERE u.id = reviewer_id) AS user
            "#,
        )
        .bind(review_id)
        .bind(reviewer_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn remove_reviewer(&self, review_id: Uuid, reviewer_id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM reviewers
            WHERE review_id = $1 AND reviewer_id = $2
            "#,
        )
        .bind(review_id)
        .bind(reviewer_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
