use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};

#[derive(Debug, Clone)]
pub struct MergeReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
}

impl MergeReviewRequest {
    pub fn new(owner: &str, repo: &str, number: i32) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
        })
    }
}
