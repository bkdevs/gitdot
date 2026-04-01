use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHttpError {
    #[error("Invalid owner name: {0}")]
    InvalidOwnerName(String),

    #[error("Invalid repository name: {0}")]
    InvalidRepositoryName(String),

    #[error("Invalid service: {0}")]
    InvalidService(String),

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

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
