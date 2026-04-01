use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::{BuildError, InputError},
};

#[derive(Debug, Clone)]
pub struct CreateBuildRequest {
    pub repo_owner: OwnerName,
    pub repo_name: RepositoryName,
    pub ref_name: String,
    pub commit_sha: String,
}

impl CreateBuildRequest {
    pub fn new(
        repo_owner: &str,
        repo_name: &str,
        ref_name: String,
        commit_sha: String,
    ) -> Result<Self, BuildError> {
        Ok(Self {
            repo_owner: OwnerName::try_new(repo_owner)
                .map_err(|e| InputError::new("owner name", e))?,
            repo_name: RepositoryName::try_new(repo_name)
                .map_err(|e| InputError::new("repository name", e))?,
            ref_name,
            commit_sha,
        })
    }
}
