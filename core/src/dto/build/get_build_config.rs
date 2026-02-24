use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::BuildError,
};

pub struct GetBuildConfigRequest {
    pub owner_name: OwnerName,
    pub repo_name: RepositoryName,
    pub ref_name: String,
}

impl GetBuildConfigRequest {
    pub fn new(owner: &str, repo: &str, ref_name: String) -> Result<Self, BuildError> {
        Ok(Self {
            owner_name: OwnerName::try_new(owner)
                .map_err(|e| BuildError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo)
                .map_err(|e| BuildError::InvalidRepositoryName(e.to_string()))?,
            ref_name,
        })
    }
}
