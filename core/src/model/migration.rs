use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Migration {
    pub id: Uuid,
    pub origin: MigrationOrigin,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct MigrationRepository {
    // Define fields here
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "migration_origin", rename_all = "lowercase")]
pub enum MigrationOrigin {
    GitHub,
}
