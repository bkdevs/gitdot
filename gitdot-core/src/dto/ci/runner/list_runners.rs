use crate::{
    dto::OwnerName,
    error::{InputError, RunnerError},
};

#[derive(Debug, Clone)]
pub struct ListRunnersRequest {
    pub owner_name: OwnerName,
}

impl ListRunnersRequest {
    pub fn new(owner_name: &str) -> Result<Self, RunnerError> {
        Ok(Self {
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| InputError::new("owner name", e))?,
        })
    }
}
