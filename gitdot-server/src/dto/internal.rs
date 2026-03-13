use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessPostReceiveServerRequest {
    pub old_sha: String,
    pub new_sha: String,
    pub ref_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessReviewServerRequest {
    pub ref_name: String,
    pub new_sha: String,
    pub pusher_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewAction {
    Created,
    Updated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessReviewServerResponse {
    pub review_number: i32,
    pub action: ReviewAction,
}

impl gitdot_api::ApiResource for ProcessReviewServerResponse {}
