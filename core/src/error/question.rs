use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum QuestionError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Question not found: {0}")]
    QuestionNotFound(String),

    #[error("Answer not found: {0}")]
    AnswerNotFound(Uuid),

    #[error("Comment not found: {0}")]
    CommentNotFound(Uuid),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
