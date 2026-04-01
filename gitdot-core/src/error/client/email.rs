use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Email error: {0}")]
    ResendError(#[from] resend_rs::Error),
}
