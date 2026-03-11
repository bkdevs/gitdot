use async_trait::async_trait;
use sqlx::{Error, PgPool};

use crate::model::Review;

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
}
