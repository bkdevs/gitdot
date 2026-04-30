use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct SlackAccount {
    pub id: Uuid,
    pub gitdot_user_id: Uuid,
    pub slack_user_id: String,
    pub slack_team_id: String,
    pub created_at: DateTime<Utc>,
}
