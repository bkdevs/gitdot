use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Migration {
    pub id: Uuid,
    pub number: i32,
    pub author_id: Uuid,
    pub origin: MigrationOrigin,
    pub status: MigrationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "migration_origin", rename_all = "lowercase")]
pub enum MigrationOrigin {
    GitHub,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "migration_status", rename_all = "lowercase")]
pub enum MigrationStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, FromRow)]
pub struct MigrationRepository {
    pub id: Uuid,
    pub migration_id: Uuid,
    pub repository_id: Option<Uuid>,
    pub full_name: String,
    pub status: MigrationRepositoryStatus,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "migration_repository_status", rename_all = "lowercase")]
pub enum MigrationRepositoryStatus {
    Pending,
    Running,
    Completed,
    Failed,
}
