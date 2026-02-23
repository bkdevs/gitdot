use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::{types::Json, FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Dag {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub task_dependencies: Json<HashMap<Uuid, Vec<Uuid>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
