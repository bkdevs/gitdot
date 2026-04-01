use thiserror::Error;

use crate::error::InputError;

#[derive(Debug, Error)]
pub enum GitHttpError {
    #[error(transparent)]
    Input(#[from] InputError),

    #[error("Failed to spawn git http-backend: {0}")]
    SpawnError(#[source] std::io::Error),

    #[error("Failed to write request body: {0}")]
    WriteError(#[source] std::io::Error),

    #[error("Failed to read response: {0}")]
    ReadError(#[source] std::io::Error),

    #[error("Git http-backend failed with exit code {code}: {stderr}")]
    ProcessFailed { code: i32, stderr: String },

    #[error("Invalid CGI response: {0}")]
    InvalidCgiResponse(String),
}
