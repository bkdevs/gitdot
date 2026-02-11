use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::dto::common::OwnerName;

#[derive(Debug, Clone, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub repo_owner: OwnerName,
    pub repo_name: String,

    pub script: String,
    // TODO: arguments?
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // TODO: logs. metadata?
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Success,
    Failure,
}
