use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Missing Authorization header")]
    MissingHeader,

    #[error("Invalid Authorization header format")]
    InvalidHeaderFormat,

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),
}
