use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Answer, Comment, Question, VoteResult};

const QUESTION_DETAILS_QUERY: &str = r#"
SELECT
    q.id,
    q.number,
    q.author_id,
    q.repository_id,
    q.title,
    q.body,
    q.upvote,
    q.impression,
    q.created_at,
    q.updated_at,

    -- User's vote on question (NULL if user_id is NULL)
    (SELECT v.value FROM votes v WHERE v.target_id = q.id AND v.user_id = $3) AS user_vote,

    -- Question Author
    (SELECT json_build_object(
        'id', u.id, 'name', u.name, 'email', u.email, 'created_at', u.created_at
    ) FROM users u WHERE u.id = q.author_id) AS author,

    -- Question Comments (with user_vote)
    COALESCE(
        (
            SELECT json_agg(
                json_build_object(
                    'id', c.id,
                    'parent_id', c.parent_id,
                    'author_id', c.author_id,
                    'body', c.body,
                    'upvote', c.upvote,
                    'user_vote', (SELECT v.value FROM votes v WHERE v.target_id = c.id AND v.user_id = $3),
                    'created_at', c.created_at,
                    'updated_at', c.updated_at,
                    'author', (SELECT json_build_object('id', cu.id, 'name', cu.name, 'email', cu.email, 'created_at', cu.created_at)
                               FROM users cu WHERE cu.id = c.author_id)
                ) ORDER BY c.created_at ASC
            )
            FROM comments c
            WHERE c.parent_id = q.id
        ),
        '[]'::json
    ) AS comments,

    -- Answers (with user_vote and nested comments with user_vote)
    COALESCE(
        (
            SELECT json_agg(
                json_build_object(
                    'id', a.id,
                    'question_id', a.question_id,
                    'author_id', a.author_id,
                    'body', a.body,
                    'upvote', a.upvote,
                    'user_vote', (SELECT v.value FROM votes v WHERE v.target_id = a.id AND v.user_id = $3),
                    'created_at', a.created_at,
                    'updated_at', a.updated_at,
                    'author', (SELECT json_build_object('id', au.id, 'name', au.name, 'email', au.email, 'created_at', au.created_at)
                               FROM users au WHERE au.id = a.author_id),
                    'comments', COALESCE(
                        (
                            SELECT json_agg(
                                json_build_object(
                                    'id', ac.id,
                                    'parent_id', ac.parent_id,
                                    'author_id', ac.author_id,
                                    'body', ac.body,
                                    'upvote', ac.upvote,
                                    'user_vote', (SELECT v.value FROM votes v WHERE v.target_id = ac.id AND v.user_id = $3),
                                    'created_at', ac.created_at,
                                    'updated_at', ac.updated_at,
                                    'author', (SELECT json_build_object('id', acu.id, 'name', acu.name, 'email', acu.email, 'created_at', acu.created_at)
                                               FROM users acu WHERE acu.id = ac.author_id)
                                ) ORDER BY ac.created_at ASC
                            )
                            FROM comments ac
                            WHERE ac.parent_id = a.id
                        ),
                        '[]'::json
                    )
                ) ORDER BY a.created_at ASC
            )
            FROM answers a
            WHERE a.question_id = q.id
        ),
        '[]'::json
    ) AS answers
FROM questions q
"#;

#[async_trait]
pub trait QuestionRepository: Send + Sync + Clone + 'static {
    async fn create_question(
        &self,
        author_id: Uuid,
        repository_id: Uuid,
        title: &str,
        body: &str,
    ) -> Result<Question, Error>;

    async fn update_question(
        &self,
        repository_id: Uuid,
        number: i32,
        title: &str,
        body: &str,
    ) -> Result<Option<Question>, Error>;

    async fn get_question(
        &self,
        repository_id: Uuid,
        number: i32,
        user_id: Option<Uuid>,
    ) -> Result<Option<Question>, Error>;

    async fn get_question_id(
        &self,
        repository_id: Uuid,
        number: i32,
    ) -> Result<Option<Uuid>, Error>;

    async fn get_questions(
        &self,
        repository_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Vec<Question>, Error>;

    async fn create_answer(
        &self,
        question_id: Uuid,
        author_id: Uuid,
        body: &str,
    ) -> Result<Answer, Error>;

    async fn update_answer(&self, id: Uuid, body: &str) -> Result<Option<Answer>, Error>;

    async fn create_comment(
        &self,
        parent_id: Uuid,
        author_id: Uuid,
        body: &str,
    ) -> Result<Comment, Error>;

    async fn update_comment(&self, id: Uuid, body: &str) -> Result<Option<Comment>, Error>;

    async fn get_question_author_id(&self, id: Uuid) -> Result<Option<Uuid>, Error>;

    async fn get_answer_author_id(&self, id: Uuid) -> Result<Option<Uuid>, Error>;

    async fn get_comment_author_id(&self, id: Uuid) -> Result<Option<Uuid>, Error>;

    /// Vote on a target (question, answer, or comment)
    /// value: 1 (upvote), -1 (downvote), 0 (remove vote)
    async fn vote(&self, user_id: Uuid, target_id: Uuid, value: i16) -> Result<VoteResult, Error>;
}

#[derive(Debug, Clone)]
pub struct QuestionRepositoryImpl {
    pool: PgPool,
}

impl QuestionRepositoryImpl {
    pub fn new(pool: PgPool) -> QuestionRepositoryImpl {
        QuestionRepositoryImpl { pool }
    }
}

#[async_trait]
impl QuestionRepository for QuestionRepositoryImpl {
    async fn create_question(
        &self,
        author_id: Uuid,
        repository_id: Uuid,
        title: &str,
        body: &str,
    ) -> Result<Question, Error> {
        let question = sqlx::query_as::<_, Question>(
            r#"
            INSERT INTO questions (number, author_id, repository_id, title, body)
            VALUES (
                COALESCE((SELECT MAX(number) FROM questions WHERE repository_id = $2), 0) + 1,
                $1, $2, $3, $4
            )
            RETURNING id, number, author_id, repository_id, title, body, upvote, impression, created_at, updated_at,
                      NULL::smallint AS user_vote, NULL AS author, NULL AS comments, NULL AS answers
            "#,
        )
        .bind(author_id)
        .bind(repository_id)
        .bind(title)
        .bind(body)
        .fetch_one(&self.pool)
        .await?;

        Ok(question)
    }

    async fn update_question(
        &self,
        repository_id: Uuid,
        number: i32,
        title: &str,
        body: &str,
    ) -> Result<Option<Question>, Error> {
        let question = sqlx::query_as::<_, Question>(
            r#"
            UPDATE questions
            SET title = $3, body = $4, updated_at = NOW()
            WHERE repository_id = $1 AND number = $2
            RETURNING id, number, author_id, repository_id, title, body, upvote, impression, created_at, updated_at,
                      NULL::smallint AS user_vote, NULL AS author, NULL AS comments, NULL AS answers
            "#,
        )
        .bind(repository_id)
        .bind(number)
        .bind(title)
        .bind(body)
        .fetch_optional(&self.pool)
        .await?;

        Ok(question)
    }

    async fn get_question(
        &self,
        repository_id: Uuid,
        number: i32,
        user_id: Option<Uuid>,
    ) -> Result<Option<Question>, Error> {
        let query = format!(
            "{} WHERE q.repository_id = $1 AND q.number = $2",
            QUESTION_DETAILS_QUERY
        );

        let question = sqlx::query_as::<_, Question>(&query)
            .bind(repository_id)
            .bind(number)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(question)
    }

    async fn get_question_id(
        &self,
        repository_id: Uuid,
        number: i32,
    ) -> Result<Option<Uuid>, Error> {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM questions WHERE repository_id = $1 AND number = $2",
        )
        .bind(repository_id)
        .bind(number)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_questions(
        &self,
        repository_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Vec<Question>, Error> {
        // Use $2 as placeholder for number to keep $3 for user_id consistent with QUESTION_DETAILS_QUERY
        let query = format!(
            "{} WHERE q.repository_id = $1 AND $2::int IS NULL ORDER BY q.created_at DESC",
            QUESTION_DETAILS_QUERY
        );

        let questions = sqlx::query_as::<_, Question>(&query)
            .bind(repository_id)
            .bind(None::<i32>)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(questions)
    }

    async fn create_answer(
        &self,
        question_id: Uuid,
        author_id: Uuid,
        body: &str,
    ) -> Result<Answer, Error> {
        let answer = sqlx::query_as::<_, Answer>(
            r#"
            INSERT INTO answers (question_id, author_id, body)
            VALUES ($1, $2, $3)
            RETURNING id, question_id, author_id, body, upvote, created_at, updated_at,
                      NULL::smallint AS user_vote, NULL AS author, NULL AS comments
            "#,
        )
        .bind(question_id)
        .bind(author_id)
        .bind(body)
        .fetch_one(&self.pool)
        .await?;

        Ok(answer)
    }

    async fn update_answer(&self, id: Uuid, body: &str) -> Result<Option<Answer>, Error> {
        let answer = sqlx::query_as::<_, Answer>(
            r#"
            UPDATE answers
            SET body = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, question_id, author_id, body, upvote, created_at, updated_at,
                      NULL::smallint AS user_vote, NULL AS author, NULL AS comments
            "#,
        )
        .bind(id)
        .bind(body)
        .fetch_optional(&self.pool)
        .await?;

        Ok(answer)
    }

    async fn create_comment(
        &self,
        parent_id: Uuid,
        author_id: Uuid,
        body: &str,
    ) -> Result<Comment, Error> {
        let comment = sqlx::query_as::<_, Comment>(
            r#"
            INSERT INTO comments (parent_id, author_id, body)
            VALUES ($1, $2, $3)
            RETURNING id, parent_id, author_id, body, upvote, created_at, updated_at,
                      NULL::smallint AS user_vote, NULL AS author
            "#,
        )
        .bind(parent_id)
        .bind(author_id)
        .bind(body)
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    async fn update_comment(&self, id: Uuid, body: &str) -> Result<Option<Comment>, Error> {
        let comment = sqlx::query_as::<_, Comment>(
            r#"
            UPDATE comments
            SET body = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, parent_id, author_id, body, upvote, created_at, updated_at,
                      NULL::smallint AS user_vote, NULL AS author
            "#,
        )
        .bind(id)
        .bind(body)
        .fetch_optional(&self.pool)
        .await?;

        Ok(comment)
    }

    async fn get_question_author_id(&self, id: Uuid) -> Result<Option<Uuid>, Error> {
        sqlx::query_scalar::<_, Uuid>("SELECT author_id FROM questions WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_answer_author_id(&self, id: Uuid) -> Result<Option<Uuid>, Error> {
        sqlx::query_scalar::<_, Uuid>("SELECT author_id FROM answers WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_comment_author_id(&self, id: Uuid) -> Result<Option<Uuid>, Error> {
        sqlx::query_scalar::<_, Uuid>("SELECT author_id FROM comments WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn vote(&self, user_id: Uuid, target_id: Uuid, value: i16) -> Result<VoteResult, Error> {
        let mut tx = self.pool.begin().await?;

        // Get existing vote if any
        let existing_vote: Option<i16> =
            sqlx::query_scalar("SELECT value FROM votes WHERE user_id = $1 AND target_id = $2")
                .bind(user_id)
                .bind(target_id)
                .fetch_optional(&mut *tx)
                .await?;

        let old_value = existing_vote.unwrap_or(0);
        let vote_delta = (value as i32) - (old_value as i32);
        let final_vote: Option<i16>;

        if value == 0 {
            // Remove vote
            sqlx::query("DELETE FROM votes WHERE user_id = $1 AND target_id = $2")
                .bind(user_id)
                .bind(target_id)
                .execute(&mut *tx)
                .await?;
            final_vote = None;
        } else if existing_vote.is_some() {
            // Update existing vote
            sqlx::query("UPDATE votes SET value = $3 WHERE user_id = $1 AND target_id = $2")
                .bind(user_id)
                .bind(target_id)
                .bind(value)
                .execute(&mut *tx)
                .await?;
            final_vote = Some(value);
        } else {
            // Insert new vote
            sqlx::query("INSERT INTO votes (user_id, target_id, value) VALUES ($1, $2, $3)")
                .bind(user_id)
                .bind(target_id)
                .bind(value)
                .execute(&mut *tx)
                .await?;
            final_vote = Some(value);
        }

        // Update denormalized upvote counter - try each table
        let new_score: i32 = if let Some(score) = sqlx::query_scalar::<_, i32>(
            "UPDATE questions SET upvote = upvote + $2 WHERE id = $1 RETURNING upvote",
        )
        .bind(target_id)
        .bind(vote_delta)
        .fetch_optional(&mut *tx)
        .await?
        {
            score
        } else if let Some(score) = sqlx::query_scalar::<_, i32>(
            "UPDATE answers SET upvote = upvote + $2 WHERE id = $1 RETURNING upvote",
        )
        .bind(target_id)
        .bind(vote_delta)
        .fetch_optional(&mut *tx)
        .await?
        {
            score
        } else if let Some(score) = sqlx::query_scalar::<_, i32>(
            "UPDATE comments SET upvote = upvote + $2 WHERE id = $1 RETURNING upvote",
        )
        .bind(target_id)
        .bind(vote_delta)
        .fetch_optional(&mut *tx)
        .await?
        {
            score
        } else {
            return Err(Error::RowNotFound);
        };

        tx.commit().await?;

        Ok(VoteResult {
            target_id,
            new_score,
            user_vote: final_vote,
        })
    }
}
