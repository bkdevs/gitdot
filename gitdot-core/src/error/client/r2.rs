use thiserror::Error;

#[derive(Debug, Error)]
pub enum R2Error {
    #[error("R2 error: failed to upload object: {0}")]
    UploadError(String),
}
