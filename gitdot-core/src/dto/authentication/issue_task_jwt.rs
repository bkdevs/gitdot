use std::time::Duration;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IssueTaskJwtRequest {
    pub task_id: Uuid,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct IssueTaskJwtResponse {
    pub token: String,
}
