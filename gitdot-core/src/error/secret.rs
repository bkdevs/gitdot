use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("Secret manager error: {0}")]
    SecretManagerError(#[from] google_cloud_secretmanager_v1::Error),

    #[error("Secret manager client builder error: {0}")]
    ClientBuilderError(#[from] google_cloud_gax::client_builder::Error),

    #[error("Missing payload for secret: {0}")]
    MissingPayload(String),

    #[error("Invalid UTF-8 in secret: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("Failed to parse secret: {0}")]
    ParseError(String),
}
