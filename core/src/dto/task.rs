mod create_task;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::common::OwnerName;
use crate::model::{Task, TaskStatus};

pub use create_task::CreateTaskRequest;

#[derive(Debug, Clone)]
pub struct TaskResponse {
    pub id: Uuid,
    pub repo_owner: OwnerName,
    pub repo_name: String,
    pub script: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            repo_owner: task.repo_owner,
            repo_name: task.repo_name,
            script: task.script,
            status: task.status,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}
