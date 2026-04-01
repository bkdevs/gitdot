use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::{InputError, UserError},
};

#[derive(Debug, Clone)]
pub struct ListUserReviewsRequest {
    pub user_name: OwnerName,
    pub viewer_id: Option<Uuid>,
    pub status: Option<String>,
    pub owner: Option<String>,
    pub repo: Option<String>,
}

impl ListUserReviewsRequest {
    pub fn new(
        user_name: &str,
        viewer_id: Option<Uuid>,
        status: Option<String>,
        owner: Option<String>,
        repo: Option<String>,
    ) -> Result<Self, UserError> {
        Ok(Self {
            user_name: OwnerName::try_new(user_name)
                .map_err(|e| InputError::new("user name", e))?,
            viewer_id,
            status,
            owner,
            repo,
        })
    }
}
