use crate::dto::{OwnerName, RepositoryName};
use crate::error::GitHttpBackendError;

#[derive(Debug, Clone)]
pub struct ReceivePackRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub content_type: String,
    pub body: Vec<u8>,
}

impl ReceivePackRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        content_type: String,
        body: Vec<u8>,
    ) -> Result<Self, GitHttpBackendError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| GitHttpBackendError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| GitHttpBackendError::InvalidRepositoryName(e.to_string()))?,
            content_type,
            body,
        })
    }
}
