use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::model::CommitFilter;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,

    #[sqlx(json(nullable))]
    pub settings: Option<UserSettings>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSettings {
    pub repos: HashMap<String, UserRepoSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRepoSettings {
    pub commit_filters: Option<Vec<CommitFilter>>,
}
