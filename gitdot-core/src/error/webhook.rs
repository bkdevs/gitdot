use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Webhook not found: {0}")]
    NotFound(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Invalid webhook URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid secret: {0}")]
    InvalidSecret(String),

    #[error("Invalid event type: {0}")]
    InvalidEventType(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
