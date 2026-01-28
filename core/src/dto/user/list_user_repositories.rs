use uuid::Uuid;

use crate::dto::OwnerName;
use crate::error::UserError;

#[derive(Debug, Clone)]
pub struct ListUserRepositoriesRequest {
    pub user_name: OwnerName,
    pub viewer_id: Option<Uuid>,
}

impl ListUserRepositoriesRequest {
    pub fn new(user_name: &str, viewer_id: Option<Uuid>) -> Result<Self, UserError> {
        Ok(Self {
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| UserError::InvalidUserName(e.to_string()))?,
            viewer_id,
        })
    }
}
