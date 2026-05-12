use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: OrganizationRole,
    pub role_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub user_name: String,
    pub org_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "core.organization_role", rename_all = "lowercase")]
pub enum OrganizationRole {
    Admin,
    Member,
}
