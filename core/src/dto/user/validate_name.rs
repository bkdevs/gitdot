use crate::dto::OwnerName;
use crate::error::UserError;

#[derive(Debug, Clone)]
pub struct ValidateNameRequest {
    pub name: OwnerName,
}

impl ValidateNameRequest {
    pub fn new(name: &str) -> Result<Self, UserError> {
        Ok(Self {
            name: OwnerName::try_new(name)
                .map_err(|e| UserError::InvalidUserName(e.to_string()))?,
        })
    }
}
