use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};


#[derive(Debug, Clone)]
pub struct UpdateReviewDiffRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub position: i32,
    pub message: Option<String>,
}

impl UpdateReviewDiffRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        position: i32,
        message: Option<String>,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::try_new(owner).map_err(|e| InputError::new("owner name", e))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            position,
            message,
        })
    }
}
