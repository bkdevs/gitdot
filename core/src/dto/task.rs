mod list_tasks;
mod update_task;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{Task, TaskStatus};

pub use list_tasks::ListTasksRequest;
pub use update_task::UpdateTaskRequest;

#[derive(Debug, Clone)]
pub struct TaskResponse {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub build_id: Uuid,
    pub name: String,
    pub command: String,
    pub status: TaskStatus,
    pub waits_for: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            repo_owner: task.repo_owner,
            repo_name: task.repo_name,
            build_id: task.build_id,
            name: task.name,
            command: task.command,
            status: task.status,
            waits_for: task.waits_for,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}
