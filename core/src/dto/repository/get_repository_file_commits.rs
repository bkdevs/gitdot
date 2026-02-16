use crate::{
    dto::{OwnerName, RepositoryName},
    error::RepositoryError,
};

#[derive(Debug, Clone)]
pub struct GetRepositoryFileCommitsRequest {
    pub name: RepositoryName,
    pub owner_name: OwnerName,
    pub ref_name: String,
    pub path: String,
    pub page: u32,
    pub per_page: u32,
}

impl GetRepositoryFileCommitsRequest {
    pub fn new(
        repo_name: &str,
        owner_name: &str,
        ref_name: String,
        path: String,
        page: u32,
        per_page: u32,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(repo_name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            ref_name,
            path,
            page,
            per_page,
        })
    }
}
