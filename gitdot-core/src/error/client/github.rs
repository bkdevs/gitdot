use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("GitHub error: {0}")]
    Other(String),

    #[error("GitHub error: {0}")]
    OctocrabError(#[from] octocrab::Error),
}
