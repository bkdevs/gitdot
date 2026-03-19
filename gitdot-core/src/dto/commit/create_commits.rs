use std::collections::HashMap;

use crate::{
    dto::{OwnerName, RepositoryName},
    error::CommitError,
};

#[derive(Debug, Clone)]
pub struct CreateCommitsRequest {
    pub owner: OwnerName,
    pub repo: RepositoryName,
    pub old_sha: String,
    pub new_sha: String,
    pub ref_name: String,
    pub review_number: Option<i32>,
    pub diff_positions: HashMap<String, i32>,
}

impl CreateCommitsRequest {
    pub fn new(
        owner: &str,
        repo: &str,
        old_sha: String,
        new_sha: String,
        ref_name: String,
        review_number: Option<i32>,
        diff_positions: HashMap<String, i32>,
    ) -> Result<Self, CommitError> {
        Ok(Self {
            owner: OwnerName::try_new(owner)
                .map_err(|e| CommitError::InvalidOwnerName(e.to_string()))?,
            repo: RepositoryName::try_new(repo)
                .map_err(|e| CommitError::InvalidOwnerName(e.to_string()))?,
            old_sha,
            new_sha,
            ref_name,
            review_number,
            diff_positions,
        })
    }
}
