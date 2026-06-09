use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::{InputError, UserError},
};

#[derive(Debug, Clone)]
pub struct UnfollowUserRequest {
    pub follower_id: Uuid,
    pub user_name: OwnerName,
}

impl UnfollowUserRequest {
    pub fn new(follower_id: Uuid, user_name: &str) -> Result<Self, UserError> {
        Ok(Self {
            follower_id,
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| InputError::new("user name", e))?,
        })
    }
}
