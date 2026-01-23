use uuid::Uuid;

use super::{OwnerName, RepositoryName};

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
    ) -> Self {
        Self {
            name: RepositoryName::try_new(name).unwrap(),
            user_id: user_id,
            owner_name: OwnerName::try_new(owner_name).unwrap(),
            owner_type: owner_type.try_into().unwrap(),
            visibility: visibility.try_into().unwrap(),
        }
    }
}
