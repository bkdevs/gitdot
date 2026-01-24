use uuid::Uuid;

use crate::dto::{OwnerName, RepositoryName};
use crate::error::RepositoryError;
use crate::model::{RepositoryOwnerType, RepositoryVisibility};

#[derive(Debug, Clone)]
pub struct CreateRepositoryRequest {
    pub name: RepositoryName,
    pub user_id: Uuid,
    pub owner_name: OwnerName,
    pub owner_type: RepositoryOwnerType,
    pub visibility: RepositoryVisibility,
}

impl CreateRepositoryRequest {
    pub fn new(
        name: &str,
        user_id: Uuid,
        owner_name: &str,
        owner_type: &str,
        visibility: &str,
    ) -> Result<Self, RepositoryError> {
        Ok(Self {
            name: RepositoryName::try_new(name)
                .map_err(|e| RepositoryError::InvalidRepositoryName(e.to_string()))?,
            user_id,
            owner_name: OwnerName::try_new(owner_name)
                .map_err(|e| RepositoryError::InvalidOwnerName(e.to_string()))?,
            owner_type: owner_type.try_into()?,
            visibility: visibility.try_into()?,
        })
    }
}
