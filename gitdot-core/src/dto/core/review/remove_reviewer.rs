use crate::error::{InputError, ReviewError};

use crate::dto::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct RemoveReviewerRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub reviewer_name: OwnerName,
}

impl RemoveReviewerRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        reviewer_name: &str,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            reviewer_name: OwnerName::try_new(reviewer_name)
                .map_err(|e| InputError::new("owner name", e))?,
        })
    }
}
