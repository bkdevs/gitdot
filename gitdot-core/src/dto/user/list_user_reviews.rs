use crate::{dto::OwnerName, error::UserError};

#[derive(Debug, Clone)]
pub struct ListUserReviewsRequest {
    pub user_name: OwnerName,
}

impl ListUserReviewsRequest {
    pub fn new(user_name: &str) -> Result<Self, UserError> {
        Ok(Self {
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| UserError::InvalidUserName(e.to_string()))?,
        })
    }
}
