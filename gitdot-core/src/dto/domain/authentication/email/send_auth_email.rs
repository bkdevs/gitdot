use crate::{dto::Email, error::SessionError};

#[derive(Debug, Clone)]
pub struct SendAuthEmailRequest {
    pub email: Email,
}

impl SendAuthEmailRequest {
    pub fn new(email: &str) -> Result<Self, SessionError> {
        Ok(Self {
            email: Email::parse(email, "email")?,
        })
    }
}
