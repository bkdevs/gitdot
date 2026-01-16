use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum OrganizationRole {
    Admin,
    Member,
}

#[derive(Debug, Clone, FromRow)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: OrganizationRole,
    pub created_at: DateTime<Utc>,
}
