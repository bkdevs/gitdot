use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IssueTaskJwtRequest {
    pub task_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct IssueTaskJwtResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub aud: String,
}
