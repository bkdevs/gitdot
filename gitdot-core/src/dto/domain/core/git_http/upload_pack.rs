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
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            content_type: GitContentType::parse(content_type, "content type")?,
            body,
        })
    }
}
