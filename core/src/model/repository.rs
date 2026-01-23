use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::error::RepositoryError;

#[derive(Debug, Clone, FromRow)]
pub struct Repository {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub owner_name: String,
    pub owner_type: RepositoryOwnerType,
    pub visibility: RepositoryVisibility,
    pub created_at: DateTime<Utc>,
}

impl Repository {
    pub fn is_owned_by_user(&self) -> bool {
        self.owner_type == RepositoryOwnerType::User
    }

    pub fn is_owned_by_organization(&self) -> bool {
        self.owner_type == RepositoryOwnerType::Organization
    }

    pub fn is_public(&self) -> bool {
        self.visibility == RepositoryVisibility::Public
    }

    pub fn is_private(&self) -> bool {
        self.visibility != RepositoryVisibility::Public
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "repository_owner_type", rename_all = "lowercase")]
pub enum RepositoryOwnerType {
    User,
    Organization,
}

impl TryFrom<&str> for RepositoryOwnerType {
    type Error = RepositoryError;

    fn try_from(owner_type: &str) -> Result<Self, Self::Error> {
        match owner_type {
            "user" => Ok(RepositoryOwnerType::User),
            "organization" => Ok(RepositoryOwnerType::Organization),
            _ => Err(RepositoryError::InvalidOwnerType(owner_type.to_string())),
        }
    }
}

impl Into<String> for RepositoryOwnerType {
    fn into(self) -> String {
        match self {
            RepositoryOwnerType::User => "user".to_string(),
            RepositoryOwnerType::Organization => "organization".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "repository_visibility", rename_all = "lowercase")]
pub enum RepositoryVisibility {
    Public,
    Private,
}

impl TryFrom<&str> for RepositoryVisibility {
    type Error = RepositoryError;

    fn try_from(visibility: &str) -> Result<Self, Self::Error> {
        match visibility {
            "public" => Ok(RepositoryVisibility::Public),
            "private" => Ok(RepositoryVisibility::Private),
            _ => Err(RepositoryError::InvalidVisibility(visibility.to_string())),
        }
    }
}

impl Into<String> for RepositoryVisibility {
    fn into(self) -> String {
        match self {
            RepositoryVisibility::Public => "public".to_string(),
            RepositoryVisibility::Private => "private".to_string(),
        }
    }
}
