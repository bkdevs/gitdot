use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::model::{Answer, Comment, Question};

const GET_QUESTION_WITH_DETAILS_QUERY: &str = r#"
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

    -- Question Author
    (SELECT json_build_object(
        'id', u.id, 'name', u.name, 'email', u.email, 'created_at', u.created_at
    ) FROM users u WHERE u.id = q.author_id) AS author,

    -- Question Comments
    COALESCE(
        (
            SELECT json_agg(
                json_build_object(
                    'id', c.id,
                    'parent_id', c.parent_id,
                    'author_id', c.author_id,
                    'body', c.body,
                    'upvote', c.upvote,
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

    -- Answers
    COALESCE(
        (
            SELECT json_agg(
                json_build_object(
                    'id', a.id,
                    'question_id', a.question_id,
                    'author_id', a.author_id,
                    'body', a.body,
                    'upvote', a.upvote,
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
    ) -> Result<Option<Question>, Error>;

    async fn get_questions(&self, repository_id: Uuid) -> Result<Vec<Question>, Error>;

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
                      NULL AS author, NULL AS comments, NULL AS answers
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
                      NULL AS author, NULL AS comments, NULL AS answers
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
    ) -> Result<Option<Question>, Error> {
        let query = format!(
            "{} WHERE q.repository_id = $1 AND q.number = $2",
            GET_QUESTION_WITH_DETAILS_QUERY
        );

        let question = sqlx::query_as::<_, Question>(&query)
            .bind(repository_id)
            .bind(number)
            .fetch_optional(&self.pool)
            .await?;

        Ok(question)
    }

    async fn get_questions(&self, repository_id: Uuid) -> Result<Vec<Question>, Error> {
        let query = format!(
            "{} WHERE q.repository_id = $1 ORDER BY q.created_at DESC",
            GET_QUESTION_WITH_DETAILS_QUERY
        );

        let questions = sqlx::query_as::<_, Question>(&query)
            .bind(repository_id)
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
                      NULL AS author, NULL AS comments
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
                      NULL AS author, NULL AS comments
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
                      NULL AS author
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
                      NULL AS author
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
}
