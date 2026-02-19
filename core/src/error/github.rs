use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubError {
    #[error("Octocrab error: {0}")]
    OctocrabError(#[from] octocrab::Error),
}
