use serde::{Deserialize, Serialize};

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
    pub pusher_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessReviewServerResponse {
    pub review_number: i64,
}

impl gitdot_api::ApiResource for ProcessReviewServerResponse {}
