use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};

#[derive(Debug, Clone)]
pub struct PublishReviewRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
}

impl PublishReviewRequest {
    pub fn new(owner: &str, repo: &str, number: i32) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
        })
    }
}
