use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{
        CommentSide, Diff, DiffStatus, Review, ReviewComment, ReviewStatus, Reviewer, Revision,
        Verdict,
    },
};

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
        'id', u.id, 'name', u.name, 'email', u.email, 'is_email_verified', u.is_email_verified,
        'provider', u.provider, 'links', u.links, 'created_at', u.created_at
    ) FROM core.users u WHERE u.id = r.author_id) AS author,

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
                                    'parent_hash', rev.parent_hash,
                                    'created_at', rev.created_at,
                                    'verdicts', COALESCE(
                                        (
                                            SELECT json_agg(
                                                json_build_object(
                                                    'id', v.id,
                                                    'diff_id', v.diff_id,
                                                    'revision_id', v.revision_id,
                                                    'reviewer_id', v.reviewer_id,
                                                    'verdict', v.verdict,
                                                    'created_at', v.created_at
                                                ) ORDER BY v.created_at ASC
                                            )
                                            FROM core.review_verdicts v
                                            WHERE v.revision_id = rev.id
                                        ),
                                        '[]'::json
                                    )
                                ) ORDER BY rev.number DESC
                            )
                            FROM core.revisions rev
                            WHERE rev.diff_id = d.id
                        ),
                        '[]'::json
                    )
                ) ORDER BY d.position ASC
            )
            FROM core.diffs d
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
                    'created_at', rv.created_at,
                    'user', (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'is_email_verified', u.is_email_verified, 'provider', u.provider, 'links', u.links, 'created_at', u.created_at)
                             FROM core.users u WHERE u.id = rv.reviewer_id)
                )
            )
            FROM core.reviewers rv
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
                    'line_number_start', c.line_number_start,
                    'line_number_end', c.line_number_end,
                    'side', c.side,
                    'resolved', c.resolved,
                    'created_at', c.created_at,
                    'updated_at', c.updated_at,
                    'author', (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'is_email_verified', u.is_email_verified, 'provider', u.provider, 'links', u.links, 'created_at', u.created_at)
                               FROM core.users u WHERE u.id = c.author_id)
                ) ORDER BY c.created_at ASC
            )
            FROM core.review_comments c
            WHERE c.review_id = r.id
        ),
        '[]'::json
    ) AS comments
FROM core.reviews r
"#;

#[async_trait]
pub trait ReviewRepository: Send + Sync + Clone + 'static {
    async fn get_review(
        &self,
        owner: &str,
        repo: &str,
        number: i32,
    ) -> Result<Option<Review>, DatabaseError>;

    async fn list_reviews(
        &self,
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Review>, DatabaseError>;

    async fn get_reviews_by_user(
        &self,
        user_name: &str,
        viewer_id: Option<Uuid>,
        status: Option<String>,
        owner: Option<String>,
        repo: Option<String>,
    ) -> Result<Vec<Review>, DatabaseError>;

    async fn create_review(
        &self,
        repository_id: Uuid,
        author_id: Uuid,
        target_branch: &str,
    ) -> Result<Review, DatabaseError>;

    async fn update_review(
        &self,
        review_id: Uuid,
        status: Option<ReviewStatus>,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<(), DatabaseError>;

    async fn create_diff(
        &self,
        review_id: Uuid,
        position: i32,
        title: &str,
        description: &str,
    ) -> Result<Diff, DatabaseError>;

    async fn update_diff(
        &self,
        diff_id: Uuid,
        status: Option<DiffStatus>,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<(), DatabaseError>;

    async fn create_revision(
        &self,
        diff_id: Uuid,
        number: i32,
        commit_hash: &str,
        parent_hash: &str,
    ) -> Result<Revision, DatabaseError>;

    async fn update_revision_sha(
        &self,
        revision_id: Uuid,
        commit_hash: &str,
        parent_hash: &str,
    ) -> Result<(), DatabaseError>;

    async fn add_reviewer(
        &self,
        review_id: Uuid,
        reviewer_id: Uuid,
    ) -> Result<Option<Reviewer>, DatabaseError>;

    async fn remove_reviewer(
        &self,
        review_id: Uuid,
        reviewer_id: Uuid,
    ) -> Result<bool, DatabaseError>;

    async fn create_verdict(
        &self,
        diff_id: Uuid,
        revision_id: Uuid,
        reviewer_id: Uuid,
        verdict: Verdict,
    ) -> Result<(), DatabaseError>;

    async fn create_comment(
        &self,
        review_id: Uuid,
        diff_id: Uuid,
        revision_id: Uuid,
        author_id: Uuid,
        body: &str,
        parent_id: Option<Uuid>,
        file_path: Option<String>,
        line_number_start: Option<i32>,
        line_number_end: Option<i32>,
        side: Option<CommentSide>,
    ) -> Result<(), DatabaseError>;

    async fn get_comment(&self, comment_id: Uuid) -> Result<Option<ReviewComment>, DatabaseError>;

    async fn update_comment(
        &self,
        comment_id: Uuid,
        body: &str,
    ) -> Result<ReviewComment, DatabaseError>;

    async fn resolve_comment(&self, comment_id: Uuid, resolved: bool) -> Result<(), DatabaseError>;
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
    ) -> Result<Option<Review>, DatabaseError> {
        let query = format!(
            "{} JOIN core.repositories repo ON r.repository_id = repo.id WHERE repo.owner_name = $1 AND repo.name = $2 AND r.number = $3",
            REVIEW_DETAILS_QUERY
        );

        let review = sqlx::query_as::<_, Review>(&query)
            .bind(owner)
            .bind(repo)
            .bind(number)
            .fetch_optional(&self.pool)
            .await?;

        Ok(review)
    }

    async fn list_reviews(
        &self,
        owner: &str,
        repo: &str,
        viewer_id: Option<Uuid>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Review>, DatabaseError> {
        let reviews = sqlx::query_as::<_, Review>(
            r#"
            SELECT
                r.id, r.repository_id, r.number, r.author_id, r.title, r.description,
                r.target_branch, r.status, r.created_at, r.updated_at,
                NULL AS author, NULL AS diffs, NULL AS reviewers, NULL AS comments
            FROM core.reviews r
            JOIN core.repositories repo ON r.repository_id = repo.id
            WHERE repo.owner_name = $1 AND repo.name = $2
                AND (r.status != 'draft' OR r.author_id = $3)
                AND r.updated_at >= $4 AND r.updated_at <= $5
            ORDER BY r.created_at DESC
            "#,
        )
        .bind(owner)
        .bind(repo)
        .bind(viewer_id.unwrap_or(Uuid::nil()))
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(reviews)
    }

    async fn get_reviews_by_user(
        &self,
        user_name: &str,
        viewer_id: Option<Uuid>,
        status: Option<String>,
        owner: Option<String>,
        repo: Option<String>,
    ) -> Result<Vec<Review>, DatabaseError> {
        let mut query = String::from(
            r#"
            SELECT
                r.id, r.repository_id, r.number, r.author_id, r.title, r.description,
                r.target_branch, r.status, r.created_at, r.updated_at,
                NULL AS author, NULL AS diffs, NULL AS reviewers, NULL AS comments
            FROM core.reviews r
            JOIN core.users u ON r.author_id = u.id
            JOIN core.repositories repo ON r.repository_id = repo.id
            WHERE u.name = $1
              AND (r.author_id = $2 OR (r.status != 'draft' AND repo.visibility = 'public'))
            "#,
        );

        let mut param_index = 3;
        if status.is_some() {
            query.push_str(&format!(
                " AND r.status = ${}::core.review_status",
                param_index
            ));
            param_index += 1;
        }
        if owner.is_some() {
            query.push_str(&format!(" AND repo.owner_name = ${}", param_index));
            param_index += 1;
        }
        if repo.is_some() {
            query.push_str(&format!(" AND repo.name = ${}", param_index));
        }
        query.push_str(" ORDER BY r.created_at DESC");

        let mut q = sqlx::query_as::<_, Review>(&query)
            .bind(user_name)
            .bind(viewer_id);
        if let Some(s) = status {
            q = q.bind(s);
        }
        if let Some(o) = owner {
            q = q.bind(o);
        }
        if let Some(r) = repo {
            q = q.bind(r);
        }
        let reviews = q.fetch_all(&self.pool).await?;

        Ok(reviews)
    }

    async fn create_review(
        &self,
        repository_id: Uuid,
        author_id: Uuid,
        target_branch: &str,
    ) -> Result<Review, DatabaseError> {
        let review = sqlx::query_as::<_, Review>(
            r#"
            WITH next_number AS (
                SELECT COALESCE(MAX(number), 0) + 1 AS number
                FROM core.reviews
                WHERE repository_id = $1
            )
            INSERT INTO core.reviews (repository_id, number, author_id, title, description, target_branch)
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
        .await?;

        Ok(review)
    }

    async fn update_review(
        &self,
        review_id: Uuid,
        status: Option<ReviewStatus>,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE core.reviews
            SET status = COALESCE($2, status),
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(review_id)
        .bind(status)
        .bind(title)
        .bind(description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_diff(
        &self,
        review_id: Uuid,
        position: i32,
        title: &str,
        description: &str,
    ) -> Result<Diff, DatabaseError> {
        let diff = sqlx::query_as::<_, Diff>(
            r#"
            INSERT INTO core.diffs (review_id, position, title, description)
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
        .await?;

        Ok(diff)
    }

    async fn update_diff(
        &self,
        diff_id: Uuid,
        status: Option<DiffStatus>,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE core.diffs
            SET status = COALESCE($2, status),
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(diff_id)
        .bind(status)
        .bind(title)
        .bind(description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_revision(
        &self,
        diff_id: Uuid,
        number: i32,
        commit_hash: &str,
        parent_hash: &str,
    ) -> Result<Revision, DatabaseError> {
        let revision = sqlx::query_as::<_, Revision>(
            r#"
            INSERT INTO core.revisions (diff_id, number, commit_hash, parent_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, diff_id, number, commit_hash, parent_hash, created_at, NULL AS verdicts
            "#,
        )
        .bind(diff_id)
        .bind(number)
        .bind(commit_hash)
        .bind(parent_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(revision)
    }

    async fn update_revision_sha(
        &self,
        revision_id: Uuid,
        commit_hash: &str,
        parent_hash: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE core.revisions
            SET commit_hash = $2,
                parent_hash = $3
            WHERE id = $1
            "#,
        )
        .bind(revision_id)
        .bind(commit_hash)
        .bind(parent_hash)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn add_reviewer(
        &self,
        review_id: Uuid,
        reviewer_id: Uuid,
    ) -> Result<Option<Reviewer>, DatabaseError> {
        let reviewer = sqlx::query_as::<_, Reviewer>(
            r#"
            INSERT INTO core.reviewers (review_id, reviewer_id)
            VALUES ($1, $2)
            ON CONFLICT (review_id, reviewer_id) DO NOTHING
            RETURNING
                id, review_id, reviewer_id, created_at,
                (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'is_email_verified', u.is_email_verified, 'provider', u.provider, 'links', u.links, 'created_at', u.created_at)
                 FROM core.users u WHERE u.id = reviewer_id) AS user
            "#,
        )
        .bind(review_id)
        .bind(reviewer_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(reviewer)
    }

    async fn remove_reviewer(
        &self,
        review_id: Uuid,
        reviewer_id: Uuid,
    ) -> Result<bool, DatabaseError> {
        let result = sqlx::query(
            r#"
            DELETE FROM core.reviewers
            WHERE review_id = $1 AND reviewer_id = $2
            "#,
        )
        .bind(review_id)
        .bind(reviewer_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn create_verdict(
        &self,
        diff_id: Uuid,
        revision_id: Uuid,
        reviewer_id: Uuid,
        verdict: Verdict,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            INSERT INTO core.review_verdicts (diff_id, revision_id, reviewer_id, verdict)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(diff_id)
        .bind(revision_id)
        .bind(reviewer_id)
        .bind(verdict)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_comment(
        &self,
        review_id: Uuid,
        diff_id: Uuid,
        revision_id: Uuid,
        author_id: Uuid,
        body: &str,
        parent_id: Option<Uuid>,
        file_path: Option<String>,
        line_number_start: Option<i32>,
        line_number_end: Option<i32>,
        side: Option<CommentSide>,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            INSERT INTO core.review_comments (review_id, diff_id, revision_id, author_id, body, parent_id, file_path, line_number_start, line_number_end, side)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(review_id)
        .bind(diff_id)
        .bind(revision_id)
        .bind(author_id)
        .bind(body)
        .bind(parent_id)
        .bind(file_path)
        .bind(line_number_start)
        .bind(line_number_end)
        .bind(side)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_comment(&self, comment_id: Uuid) -> Result<Option<ReviewComment>, DatabaseError> {
        let comment = sqlx::query_as::<_, ReviewComment>(
            r#"
            SELECT
                c.id, c.review_id, c.diff_id, c.revision_id, c.author_id, c.parent_id,
                c.body, c.file_path, c.line_number_start, c.line_number_end, c.side,
                c.resolved, c.created_at, c.updated_at,
                (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'is_email_verified', u.is_email_verified, 'provider', u.provider, 'links', u.links, 'created_at', u.created_at)
                 FROM core.users u WHERE u.id = c.author_id) AS author
            FROM core.review_comments c
            WHERE c.id = $1
            "#,
        )
        .bind(comment_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(comment)
    }

    async fn update_comment(
        &self,
        comment_id: Uuid,
        body: &str,
    ) -> Result<ReviewComment, DatabaseError> {
        let comment = sqlx::query_as::<_, ReviewComment>(
            r#"
            UPDATE core.review_comments
            SET body = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING
                id, review_id, diff_id, revision_id, author_id, parent_id,
                body, file_path, line_number_start, line_number_end, side,
                resolved, created_at, updated_at,
                (SELECT json_build_object('id', u.id, 'name', u.name, 'email', u.email, 'is_email_verified', u.is_email_verified, 'provider', u.provider, 'links', u.links, 'created_at', u.created_at)
                 FROM core.users u WHERE u.id = author_id) AS author
            "#,
        )
        .bind(comment_id)
        .bind(body)
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    async fn resolve_comment(&self, comment_id: Uuid, resolved: bool) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE core.review_comments
            SET resolved = $2, updated_at = NOW()
            WHERE id = $1 OR parent_id = $1
            "#,
        )
        .bind(comment_id)
        .bind(resolved)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
