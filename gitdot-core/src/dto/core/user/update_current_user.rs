use uuid::Uuid;

use crate::{
    dto::OwnerName,
    error::{InputError, UserError},
};

#[derive(Debug, Clone)]
pub struct UpdateCurrentUserRequest {
    pub user_id: Uuid,
    pub name: Option<OwnerName>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub website: Option<String>,
    pub company: Option<String>,
}

impl UpdateCurrentUserRequest {
    pub fn new(
        user_id: Uuid,
        name: Option<&str>,
        location: Option<String>,
        readme: Option<String>,
        website: Option<String>,
        company: Option<String>,
    ) -> Result<Self, UserError> {
        Ok(Self {
            user_id,
            name: name
                .map(|n| OwnerName::try_new(n).map_err(|e| InputError::new("user name", e)))
                .transpose()?,
            location,
            readme,
            website,
            company,
        })
    }
}
