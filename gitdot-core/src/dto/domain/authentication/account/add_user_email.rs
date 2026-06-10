use uuid::Uuid;

use crate::{dto::Email, error::AccountError};

#[derive(Debug, Clone)]
pub struct AddUserEmailRequest {
    pub user_id: Uuid,
    pub email: Email,
}

impl AddUserEmailRequest {
    pub fn new(user_id: Uuid, email: &str) -> Result<Self, AccountError> {
        Ok(Self {
            user_id,
            email: Email::parse(email, "email")?,
        })
    }
}
