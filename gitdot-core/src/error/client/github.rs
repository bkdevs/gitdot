use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("GitHub error: {0}")]
    Other(String),

    #[error("Invalid GitHub webhook signature")]
    InvalidSignature,

    #[error("Invalid GitHub install state")]
    InvalidState,

    #[error("Unauthorized for this GitHub installation")]
    Unauthorized,

    #[error("GitHub error: {0}")]
    OctocrabError(#[from] octocrab::Error),
}
