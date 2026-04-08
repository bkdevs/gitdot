use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::model::CommitFilter;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub is_email_verified: bool,
    pub provider: AuthProvider,
    pub created_at: DateTime<Utc>,
    pub location: Option<String>,
    pub readme: Option<String>,
    #[sqlx(json)]
    pub links: Vec<String>,
    pub company: Option<String>,

    #[sqlx(json(nullable), default)]
    pub settings: Option<UserSettings>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "core.auth_provider", rename_all = "lowercase")]
pub enum AuthProvider {
    Email,
    GitHub,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSettings {
    pub repos: HashMap<String, UserRepoSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRepoSettings {
    pub commit_filters: Option<Vec<CommitFilter>>,
}
