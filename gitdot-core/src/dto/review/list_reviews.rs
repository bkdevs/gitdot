use crate::error::ReviewError;

use super::super::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct ListReviewsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
}

impl ListReviewsRequest {
    pub fn new(owner: &str, repo: &str) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| ReviewError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| ReviewError::InvalidRepositoryName(e.to_string()))?,
        })
    }
}
