use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum QuestionError {
    #[error("Question not found: {0}")]
    QuestionNotFound(Uuid),

    #[error("Answer not found: {0}")]
    AnswerNotFound(Uuid),

    #[error("Comment not found: {0}")]
    CommentNotFound(Uuid),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(Uuid),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
