use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuestionError {
    #[error("Question not found: {0}")]
    QuestionNotFound(String),

    #[error("Answer not found: {0}")]
    AnswerNotFound(String),

    #[error("Comment not found: {0}")]
    CommentNotFound(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
