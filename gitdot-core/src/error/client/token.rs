use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Token signing error: {0}")]
    SigningError(String),
}
