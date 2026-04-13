use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Image error: failed to decode: {0}")]
    DecodeError(String),

    #[error("Image error: failed to encode: {0}")]
    EncodeError(String),

    #[error("Image error: failed to generate avatar: {0}")]
    GenerateError(String),

    #[error("Image error: failed to process")]
    SpawnError,
}
