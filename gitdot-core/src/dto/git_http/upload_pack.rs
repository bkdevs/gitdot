use tokio::io::AsyncRead;

use crate::{
    dto::{GitContentType, OwnerName, RepositoryName},
    error::GitHttpError,
};

pub struct UploadPackRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub content_type: GitContentType,
    pub body: Box<dyn AsyncRead + Unpin + Send>,
}

impl UploadPackRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        content_type: &str,
        body: Box<dyn AsyncRead + Unpin + Send>,
    ) -> Result<Self, GitHttpError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| GitHttpError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| GitHttpError::InvalidRepositoryName(e.to_string()))?,
            content_type: GitContentType::try_new(content_type.to_string())
                .map_err(|e| GitHttpError::InvalidContentType(e.to_string()))?,
            body,
        })
    }
}
