use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::error::TaskError;

#[derive(Debug, Clone, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub dag_id: Uuid,
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
    Assigned,
    Running,
    Success,
    Failure,
}

impl TryFrom<&str> for TaskStatus {
    type Error = TaskError;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        match status {
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
            TaskStatus::Pending => "pending".to_string(),
            TaskStatus::Assigned => "assigned".to_string(),
            TaskStatus::Running => "running".to_string(),
            TaskStatus::Success => "success".to_string(),
            TaskStatus::Failure => "failure".to_string(),
        }
    }
}
