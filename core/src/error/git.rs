use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git2Error(#[from] git2::Error),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

}
