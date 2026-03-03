use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IssueTaskJwtRequest {
    pub task_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct IssueTaskJwtResponse {
    pub token: String,
}
