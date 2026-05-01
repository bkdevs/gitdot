use thiserror::Error;

use crate::error::{DatabaseError, GitError, InputError, KafkaError, NotFoundError};

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    GitError(#[from] GitError),

    #[error(transparent)]
    KafkaError(#[from] KafkaError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
