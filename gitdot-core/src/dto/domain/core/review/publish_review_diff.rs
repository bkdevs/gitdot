use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};

#[derive(Debug, Clone)]
pub struct PublishReviewDiffRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub position: i32,
}

impl PublishReviewDiffRequest {
    pub fn new(owner: &str, repo: &str, number: i32, position: i32) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            position,
        })
    }
}
