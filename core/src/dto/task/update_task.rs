use uuid::Uuid;

use crate::{error::TaskError, model::task::TaskStatus};

#[derive(Debug, Clone)]
pub struct UpdateTaskRequest {
    pub id: Uuid,
    pub status: TaskStatus,
}

impl UpdateTaskRequest {
    pub fn new(id: Uuid, status: &str) -> Result<Self, TaskError> {
        Ok(Self {
            id,
            status: TaskStatus::try_from(status)?,
        })
    }
}
