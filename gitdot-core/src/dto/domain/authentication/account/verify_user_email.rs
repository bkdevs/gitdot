use uuid::Uuid;

use crate::{
    dto::{Email, UserCode},
    error::{AccountError, InputError},
};

#[derive(Debug, Clone)]
pub struct VerifyUserEmailRequest {
    pub user_id: Uuid,
    pub email: Email,
    pub code: UserCode,
}

impl VerifyUserEmailRequest {
    pub fn new(user_id: Uuid, email: &str, code: &str) -> Result<Self, AccountError> {
        Ok(Self {
            user_id,
            email: Email::parse(email, "email")?,
            code: UserCode::try_new(code).map_err(|e| InputError::new("code", e.to_string()))?,
        })
    }
}
