use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,

    pub display_name: Option<String>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,

    #[sqlx(json(nullable))]
    pub members: Option<Vec<OrganizationMember>>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,

    pub role: OrganizationRole,
    pub role_description: Option<String>,

    pub created_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "core.organization_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum OrganizationRole {
    Admin,
    Member,
}
