use crate::{dto::OwnerName, error::UserError};

#[derive(Debug, Clone)]
pub struct HasUserRequest {
    pub name: OwnerName,
}

impl HasUserRequest {
    pub fn new(name: &str) -> Result<Self, UserError> {
        Ok(Self {
            name: OwnerName::parse(name, "user name")?,
        })
    }
}
