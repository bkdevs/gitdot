use uuid::Uuid;

use crate::dto::OwnerName;
use crate::error::UserError;

#[derive(Debug, Clone)]
pub struct UpdateCurrentUserRequest {
    pub user_id: Uuid,
    pub name: OwnerName,
}

impl UpdateCurrentUserRequest {
    pub fn new(user_id: Uuid, name: &str) -> Result<Self, UserError> {
        Ok(Self {
            user_id,
            name: OwnerName::try_new(name)
                .map_err(|e| UserError::InvalidUserName(e.to_string()))?,
        })
    }
}
