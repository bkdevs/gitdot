use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IssueTaskTokenRequest {
    pub task_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct IssueTaskTokenResponse {
    pub token: String,
}
