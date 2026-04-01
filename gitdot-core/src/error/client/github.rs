use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Octocrab error: {0}")]
    OctocrabError(#[from] octocrab::Error),
}
