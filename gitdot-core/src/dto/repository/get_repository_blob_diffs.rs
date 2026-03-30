use std::collections::HashMap;

use crate::{
    dto::{OwnerName, RepositoryDiffFileResponse, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryBlobDiffsRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub commit_shas: Vec<String>,
    pub path: String,
}

impl GetRepositoryBlobDiffsRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        commit_shas: Vec<String>,
        path: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            commit_shas,
            path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryBlobDiffsResponse {
    pub diffs: HashMap<String, RepositoryDiffFileResponse>,
}
