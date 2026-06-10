use crate::{
    dto::{OwnerName, common::RunnerName},
    error::RunnerError,
};

#[derive(Debug, Clone)]
pub struct DeleteRunnerRequest {
    pub owner_name: OwnerName,
    pub name: RunnerName,
}

impl DeleteRunnerRequest {
    pub fn new(owner_name: &str, name: &str) -> Result<Self, RunnerError> {
        Ok(Self {
            owner_name: OwnerName::parse(owner_name, "owner name")?,
            name: RunnerName::parse(name, "runner name")?,
        })
    }
}
