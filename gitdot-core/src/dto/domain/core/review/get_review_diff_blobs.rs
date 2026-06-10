use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
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
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
            position,
            revision,
            compare_to,
        })
    }
}
