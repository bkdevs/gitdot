use crate::{
    dto::OwnerName,
    error::{InputError, UserError},
};

#[derive(Debug, Clone)]
pub struct ListUserCommitsRequest {
    pub user_name: OwnerName,
}

impl ListUserCommitsRequest {
    pub fn new(user_name: &str) -> Result<Self, UserError> {
        Ok(Self {
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| InputError::new("user name", e))?,
        })
    }
}
