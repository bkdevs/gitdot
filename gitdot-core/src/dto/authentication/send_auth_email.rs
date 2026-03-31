use crate::{dto::Email, error::AuthenticationError};

#[derive(Debug, Clone)]
pub struct SendAuthEmailRequest {
    pub email: Email,
}

impl SendAuthEmailRequest {
    pub fn new(email: &str) -> Result<Self, AuthenticationError> {
        Ok(Self {
            email: Email::try_new(email)
                .map_err(|e| AuthenticationError::InvalidEmail(e.to_string()))?,
        })
    }
}
