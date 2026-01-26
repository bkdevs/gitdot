use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::dto::VoteResponse;

#[derive(Debug, Clone, Deserialize)]
pub struct VoteServerRequest {
    pub value: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VoteServerResponse {
    pub target_id: Uuid,
    pub score: i32,
    pub user_vote: Option<i16>,
}

impl From<VoteResponse> for VoteServerResponse {
    fn from(response: VoteResponse) -> Self {
        Self {
            target_id: response.target_id,
            score: response.score,
            user_vote: response.user_vote,
        }
    }
}
