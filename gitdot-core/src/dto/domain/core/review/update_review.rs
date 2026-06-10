use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::ReviewError,
};

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
            owner: OwnerName::parse(owner, "owner name")?,
            repo: RepositoryName::parse(repo, "repository name")?,
            number,
            title,
            description,
        })
    }
}
