use crate::error::{InputError, ReviewError};

use super::super::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct AddReviewerRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub user_name: OwnerName,
}

impl AddReviewerRequest {
    pub fn new(owner: &str, repo: &str, number: i32, user_name: &str) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| InputError::new("owner name", e))?,
        })
    }
}
