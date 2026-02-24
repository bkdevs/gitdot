use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::{FromRow, types::Json};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Build {
    pub id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub task_dependencies: Json<HashMap<Uuid, Vec<Uuid>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
