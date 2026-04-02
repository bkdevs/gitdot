use crate::{
    dto::Email,
    error::{AuthenticationError, InputError},
};

#[derive(Debug, Clone)]
pub struct SendAuthEmailRequest {
    pub email: Email,
}

impl SendAuthEmailRequest {
    pub fn new(email: &str) -> Result<Self, AuthenticationError> {
        Ok(Self {
            email: Email::try_new(email).map_err(|e| InputError::new("email", e))?,
        })
    }
}
