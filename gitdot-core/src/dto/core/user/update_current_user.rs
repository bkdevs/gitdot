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
    pub links: Option<Vec<String>>,
    pub company: Option<String>,
    pub image: Option<String>,
}

impl UpdateCurrentUserRequest {
    pub fn new(
        user_id: Uuid,
        name: Option<&str>,
        location: Option<String>,
        readme: Option<String>,
        links: Option<Vec<String>>,
        company: Option<String>,
        image: Option<String>,
    ) -> Result<Self, UserError> {
        Ok(Self {
            user_id,
            name: name
                .map(|n| OwnerName::try_new(n).map_err(|e| InputError::new("user name", e)))
                .transpose()?,
            location,
            readme,
            links,
            company,
            image,
        })
    }

    pub fn with_image(user_id: Uuid, image: String) -> Self {
        Self {
            user_id,
            name: None,
            location: None,
            readme: None,
            links: None,
            company: None,
            image: Some(image),
        }
    }
}
