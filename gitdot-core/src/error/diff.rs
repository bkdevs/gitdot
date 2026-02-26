use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiffError {
    #[error("Diff error: {0}")]
    DifftasticFailed(String),
}
