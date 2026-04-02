use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Missing authorization header")]
    MissingHeader,

    #[error("Invalid authorization header format")]
    InvalidHeaderFormat,

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("JWT signing error: {0}")]
    SigningError(String),
}
