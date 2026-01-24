use crate::dto::{OwnerName, RepositoryName};
use crate::error::RepositoryError;

#[derive(Debug, Clone)]
pub struct GetRepositoryTreeRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
}

impl GetRepositoryTreeRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RepositoryTreeResponse {
    pub name: String,
    pub owner: String,
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryTreeEntry>,
}

#[derive(Debug, Clone)]
pub struct RepositoryTreeEntry {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    // pub commit: RepositoryCommit,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub preview: Option<String>,
}
