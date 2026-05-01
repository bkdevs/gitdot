use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::SlackAccount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackStatePayload {
    pub gitdot_user_id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub slack_channel_id: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Debug, Clone)]
pub struct LinkSlackAccountRequest {
    pub gitdot_user_id: Uuid,
    pub state: String,
}

impl LinkSlackAccountRequest {
    pub fn new(gitdot_user_id: Uuid, state: String) -> Self {
        Self {
            gitdot_user_id,
            state,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinkSlackAccountResponse {
    pub id: Uuid,
    pub gitdot_user_id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub created_at: DateTime<Utc>,
}

impl From<SlackAccount> for LinkSlackAccountResponse {
    fn from(account: SlackAccount) -> Self {
        Self {
            id: account.id,
            gitdot_user_id: account.gitdot_user_id,
            slack_user_id: account.slack_user_id,
            slack_team_id: account.slack_team_id,
            created_at: account.created_at,
        }
    }
}
