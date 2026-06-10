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
            owner_name: OwnerName::parse(owner_name, "owner name")?,
            name: RunnerName::parse(name, "runner name")?,
        })
    }
}
