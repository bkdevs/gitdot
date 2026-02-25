use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::error::TaskError;

#[derive(Debug, Clone, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,

    pub build_id: Uuid,
    pub s2_uri: String,
    pub waits_for: Vec<Uuid>,
    pub status: TaskStatus,

    pub name: String,
    pub command: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Blocked,
    Pending,
    Assigned,
    Running,
    Success,
    Failure,
}

impl TryFrom<&str> for TaskStatus {
    type Error = TaskError;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        match status {
            "blocked" => Ok(TaskStatus::Blocked),
            "pending" => Ok(TaskStatus::Pending),
            "assigned" => Ok(TaskStatus::Assigned),
            "running" => Ok(TaskStatus::Running),
            "success" => Ok(TaskStatus::Success),
            "failure" => Ok(TaskStatus::Failure),
            _ => Err(TaskError::InvalidStatus(status.to_string())),
        }
    }
}

impl Into<String> for TaskStatus {
    fn into(self) -> String {
        match self {
            TaskStatus::Blocked => "blocked".to_string(),
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::Assigned => "assigned".to_string(),
            TaskStatus::Running => "running".to_string(),
            TaskStatus::Success => "success".to_string(),
            TaskStatus::Failure => "failure".to_string(),
        }
    }
}
