use nutype::nutype;
use uuid::Uuid;

use crate::models::{RepositoryOwnerType, RepositoryVisibility};

#[derive(Debug, Clone)]
pub struct CreateRepositoryRequest {
    pub name: RepositoryName,
    pub user_id: Uuid,
    pub owner_name: String,
    pub owner_type: RepositoryOwnerType,
    pub visibility: RepositoryVisibility,
}

impl CreateRepositoryRequest {
    pub fn new(
        name: String,
        user_id: Uuid,
        owner_name: String,
        owner_type: String,
        visibility: String,
    ) -> Self {
        Self {
            name: RepositoryName::try_new(name).unwrap(),
            user_id: user_id,
            owner_name: owner_name,
            owner_type: owner_type.into(),
            visibility: visibility.into(),
        }
    }
}

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 100),
    derive(
        Debug,
        Clone,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        TryFrom,
        AsRef,
        Deref,
    )
)]
pub struct RepositoryName(String);
