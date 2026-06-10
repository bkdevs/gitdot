use tokio::io::AsyncRead;

use crate::{
    dto::{GitContentType, OwnerName, RepositoryName},
    error::{GitHttpError, InputError},
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
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            content_type: GitContentType::try_new(content_type.to_string())
                .map_err(|e| InputError::new("content type", e))?,
            body,
        })
    }
}
