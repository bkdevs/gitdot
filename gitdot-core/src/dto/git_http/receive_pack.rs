use tokio::io::AsyncRead;

use uuid::Uuid;

use crate::{
    dto::{GitContentType, OwnerName, RepositoryName},
    error::{GitHttpError, InputError},
};

pub struct ReceivePackRequest {
    pub pusher_id: Option<Uuid>,
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub content_type: GitContentType,
    pub body: Box<dyn AsyncRead + Unpin + Send>,
}

impl ReceivePackRequest {
    pub fn new(
        pusher_id: Option<Uuid>,
        owner: &str,
        repo: &str,
        content_type: &str,
        body: Box<dyn AsyncRead + Unpin + Send>,
    ) -> Result<Self, GitHttpError> {
        Ok(Self {
            pusher_id,
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            content_type: GitContentType::try_new(content_type.to_string())
                .map_err(|e| InputError::new("content type", e))?,
            body,
        })
    }
}
