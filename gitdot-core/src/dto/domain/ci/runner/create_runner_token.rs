use crate::{
    dto::{OwnerName, common::RunnerName},
    error::RunnerError,
};

#[derive(Debug, Clone)]
pub struct CreateRunnerTokenRequest {
    pub owner_name: OwnerName,
    pub runner_name: RunnerName,
}

impl CreateRunnerTokenRequest {
    pub fn new(owner_name: &str, name: &str) -> Result<Self, RunnerError> {
        Ok(Self {
            owner_name: OwnerName::parse(owner_name, "owner name")?,
            runner_name: RunnerName::parse(name, "runner name")?,
        })
    }
}

pub struct CreateRunnerTokenResponse {
    pub token: String,
}
