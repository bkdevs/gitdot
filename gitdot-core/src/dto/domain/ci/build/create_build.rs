use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::BuildError,
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
            repo_owner: OwnerName::parse(repo_owner, "owner name")?,
            repo_name: RepositoryName::parse(repo_name, "repository name")?,
            ref_name,
            commit_sha,
        })
    }
}
