use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use super::RepositoryOwnerType;

#[derive(Debug, Clone, FromRow)]
pub struct Migration {
    pub id: Uuid,
    pub number: i32,
    pub author_id: Uuid,

    pub origin_service: MigrationOriginService,
    pub origin: String,
    pub origin_type: RepositoryOwnerType,
    pub destination: String,
    pub destination_type: RepositoryOwnerType,

    pub status: MigrationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "migration_origin_service", rename_all = "lowercase")]
pub enum MigrationOriginService {
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

    pub origin_full_name: String,
    pub destination_full_name: String,

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
