use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct GitHubInstallation {
    pub id: Uuid,
    pub installation_id: i64,
    pub owner_id: Uuid,
    pub r#type: GitHubInstallationType,
    pub github_login: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "github_installation_type", rename_all = "lowercase")]
pub enum GitHubInstallationType {
    User,
    Organization,
}
