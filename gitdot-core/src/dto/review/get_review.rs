use crate::error::{InputError, ReviewError};

use super::super::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct GetReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
}

impl GetReviewRequest {
    pub fn new(owner: &str, repo: &str, number: i32) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
        })
    }

    pub fn get_review_path(&self) -> String {
        format!(
            "{}/{}/review/{}",
            self.owner.as_ref(),
            self.repo.as_ref(),
            self.number
        )
    }
}
