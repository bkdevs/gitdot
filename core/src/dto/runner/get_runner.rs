use crate::{
    dto::{OwnerName, common::RunnerName},
    error::RunnerError,
};

#[derive(Debug, Clone)]
pub struct GetRunnerRequest {
    pub owner_name: OwnerName,
    pub name: RunnerName,
}

impl GetRunnerRequest {
    pub fn new(owner_name: &str, name: &str) -> Result<Self, RunnerError> {
        Ok(Self {
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RunnerError::InvalidOwnerName(e.to_string()))?,
            name: RunnerName::try_new(name)
                .map_err(|e| RunnerError::InvalidRunnerName(e.to_string()))?,
        })
    }
}
