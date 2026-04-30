use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlackAccountResource {
    pub id: Uuid,
    pub gitdot_user_id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub created_at: DateTime<Utc>,
}
