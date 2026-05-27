use uuid::Uuid;

use crate::{
    dto::Email,
    error::{AccountError, InputError},
};

#[derive(Debug, Clone)]
pub struct ResendVerificationCodeRequest {
    pub user_id: Uuid,
    pub email: Email,
}

impl ResendVerificationCodeRequest {
    pub fn new(user_id: Uuid, email: &str) -> Result<Self, AccountError> {
        Ok(Self {
            user_id,
            email: Email::try_new(email).map_err(|e| InputError::new("email", e))?,
        })
    }
}
