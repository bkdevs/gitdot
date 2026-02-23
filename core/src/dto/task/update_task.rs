use uuid::Uuid;

use crate::{error::TaskError, model::TaskStatus};

#[derive(Debug, Clone)]
pub struct UpdateTaskRequest {
    pub id: Uuid,
    pub status: TaskStatus,
}

impl UpdateTaskRequest {
    pub fn new(id: Uuid, status: &str) -> Result<Self, TaskError> {
        let status = TaskStatus::try_from(status)?;
        match status {
            TaskStatus::Success | TaskStatus::Failure => Ok(Self { id, status }),
            _ => Err(TaskError::InvalidStatus(status.into())),
        }
    }
}
