use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git2Error(git2::Error),

    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

impl From<git2::Error> for GitError {
    fn from(e: git2::Error) -> Self {
        if e.code() == git2::ErrorCode::NotFound {
            GitError::NotFound(e.message().to_string())
        } else {
            GitError::Git2Error(e)
        }
    }
}
