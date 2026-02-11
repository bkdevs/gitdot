use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::dto::common::OwnerName;

#[derive(Debug, Clone, FromRow)]
pub struct Dag {
    pub id: Uuid,
    pub repo_owner: OwnerName,
    pub repo_name: String,
    pub task_ids: Vec<Uuid>,
    // TODO: add dependencies list.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // TODO: need status or always compute? actually i do think we need some derived state here.
}
