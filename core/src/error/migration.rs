use thiserror::Error;

use super::GitHubError;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("GitHub error: {0}")]
    GitHubError(#[from] GitHubError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
