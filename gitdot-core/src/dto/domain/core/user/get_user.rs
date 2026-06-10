use crate::{dto::OwnerName, error::UserError};

#[derive(Debug, Clone)]
pub struct GetUserRequest {
    pub user_name: OwnerName,
}

impl GetUserRequest {
    pub fn new(user_name: &str) -> Result<Self, UserError> {
        Ok(Self {
            user_name: OwnerName::parse(user_name, "user name")?,
        })
    }
}
