use uuid::Uuid;

use crate::{
    dto::Email,
    error::{InputError, UserError},
};

#[derive(Debug, Clone)]
pub struct AddUserEmailRequest {
    pub user_id: Uuid,
    pub email: Email,
}

impl AddUserEmailRequest {
    pub fn new(user_id: Uuid, email: &str) -> Result<Self, UserError> {
        Ok(Self {
            user_id,
            email: Email::try_new(email).map_err(|e| InputError::new("email", e))?,
        })
    }
}
