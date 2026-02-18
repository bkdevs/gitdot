use crate::{
    dto::common::{OwnerName, RunnerName},
    error::RunnerError,
    model::RunnerOwnerType,
};

#[derive(Debug, Clone)]
pub struct GetRunnerRequest {
    pub owner_name: OwnerName,
    pub owner_type: RunnerOwnerType,
    pub name: RunnerName,
}

impl GetRunnerRequest {
    pub fn new(owner_name: &str, owner_type: &str, name: &str) -> Result<Self, RunnerError> {
        Ok(Self {
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RunnerError::InvalidOwnerName(e.to_string()))?,
            owner_type: owner_type.try_into()?,
            name: RunnerName::try_new(name)
                .map_err(|e| RunnerError::InvalidRunnerName(e.to_string()))?,
        })
    }
}
