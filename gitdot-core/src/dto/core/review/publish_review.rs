use crate::error::{InputError, ReviewError};

use crate::dto::common::{OwnerName, RepositoryName};

#[derive(Debug, Clone)]
pub struct PublishReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub diffs: Vec<DiffUpdateRequest>,
}

#[derive(Debug, Clone)]
pub struct DiffUpdateRequest {
    pub position: i32,
    pub message: Option<String>,
}

impl PublishReviewRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        title: Option<String>,
        description: Option<String>,
        diffs: Vec<DiffUpdateRequest>,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            title,
            description,
            diffs,
        })
    }
}
