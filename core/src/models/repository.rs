use chrono::{DateTime, Utc};
use sqlx::FromRow;
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

#[derive(Debug, Clone)]
pub enum RepositoryOwnerType {
    User,
    Organization,
}

#[derive(Debug, Clone)]
pub enum RepositoryVisibility {
    Public,
    Private,
}
