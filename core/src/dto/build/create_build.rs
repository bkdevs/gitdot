use gitdot_config::ci::BuildTrigger;

use crate::{
    dto::common::{OwnerName, RepositoryName},
    error::BuildError,
};

#[derive(Debug, Clone)]
pub struct CreateBuildRequest {
    pub repo_owner: OwnerName,
    pub repo_name: RepositoryName,
    pub trigger: BuildTrigger,
    pub commit_sha: String,
}

impl CreateBuildRequest {
    pub fn new(
        repo_owner: &str,
        repo_name: &str,
        trigger: &str,
        commit_sha: String,
    ) -> Result<Self, BuildError> {
        Ok(Self {
            repo_owner: OwnerName::try_new(repo_owner)
                .map_err(|e| BuildError::InvalidOwnerName(e.to_string()))?,
            repo_name: RepositoryName::try_new(repo_name)
                .map_err(|e| BuildError::InvalidRepositoryName(e.to_string()))?,
            trigger: BuildTrigger::try_from(trigger.to_string())
                .map_err(BuildError::InvalidTrigger)?,
            commit_sha,
        })
    }
}
