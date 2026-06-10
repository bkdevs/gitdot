use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{InputError, ReviewError},
};

#[derive(Debug, Clone)]
pub struct GetReviewDiffBlobsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub number: i32,
    pub position: i32,
    pub revision: Option<i32>,
    pub compare_to: Option<i32>,
}

impl GetReviewDiffBlobsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        number: i32,
        position: i32,
        revision: Option<i32>,
        compare_to: Option<i32>,
    ) -> Result<Self, ReviewError> {
        Ok(Self {
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| InputError::new("repository name", e))?,
            number,
            position,
            revision,
            compare_to,
        })
    }
}
