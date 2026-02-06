use tokio::io::AsyncRead;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::GitHttpError;

pub struct UploadPackRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub content_type: String,
    pub body: Box<dyn AsyncRead + Unpin + Send>,
}

impl UploadPackRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        content_type: String,
        body: Box<dyn AsyncRead + Unpin + Send>,
    ) -> Result<Self, GitHttpError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| GitHttpError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| GitHttpError::InvalidRepositoryName(e.to_string()))?,
            content_type,
            body,
        })
    }
}
