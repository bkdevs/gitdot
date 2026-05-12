use thiserror::Error;

use crate::error::{
    DatabaseError, GitError, GitHubError, InputError, KafkaError, NotFoundError, SlackBotError,
};

#[derive(Debug, Error)]
pub enum WebhookError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    GitError(#[from] GitError),

    #[error(transparent)]
    GitHubError(#[from] GitHubError),

    #[error(transparent)]
    KafkaError(#[from] KafkaError),

    #[error(transparent)]
    SlackBotError(#[from] SlackBotError),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
