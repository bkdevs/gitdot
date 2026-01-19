use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

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

impl From<String> for RepositoryOwnerType {
    fn from(owner_type: String) -> Self {
        match owner_type.as_str() {
            "user" => RepositoryOwnerType::User,
            "organization" => RepositoryOwnerType::Organization,
            _ => panic!("Invalid owner type"),
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

impl From<String> for RepositoryVisibility {
    fn from(visibility: String) -> Self {
        match visibility.as_str() {
            "public" => RepositoryVisibility::Public,
            "private" => RepositoryVisibility::Private,
            _ => panic!("Invalid visibility"),
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
