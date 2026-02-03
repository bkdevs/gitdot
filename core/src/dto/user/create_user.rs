use crate::dto::OwnerName;
use crate::error::UserError;

#[derive(Debug, Clone)]
pub struct CreateUserRequest {
    pub name: OwnerName,
    pub email: String,
    pub password: String,
}

impl CreateUserRequest {
    pub fn new(name: &str, email: &str, password: &str) -> Result<Self, UserError> {
        Ok(Self {
            name: OwnerName::try_new(name)
                .map_err(|e| UserError::InvalidUserName(e.to_string()))?,
            email: email.to_string(),
            password: password.to_string(),
        })
    }
}
