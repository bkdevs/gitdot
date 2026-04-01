use crate::error::{InputError, ReviewError};

use super::super::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct UpdateReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl UpdateReviewRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            title,
            description,
        })
    }
}
