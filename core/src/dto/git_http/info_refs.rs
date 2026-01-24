use nutype::nutype;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::GitHttpError;

#[derive(Debug, Clone)]
pub struct InfoRefsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub service: GitService,
}

impl InfoRefsRequest {
    pub fn new(owner: &str, repo: &str, service: &str) -> Result<Self, GitHttpError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| GitHttpError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| GitHttpError::InvalidRepositoryName(e.to_string()))?,
            service: GitService::try_new(service.to_string())
                .map_err(|e| GitHttpError::InvalidService(e.to_string()))?,
        })
    }
}

#[nutype(
    validate(predicate = |s| s == "git-upload-pack" || s == "git-receive-pack"),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct GitService(String);
