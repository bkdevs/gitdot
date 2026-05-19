use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub is_email_verified: bool,
    pub provider: AuthProvider,
    pub created_at: DateTime<Utc>,

    // user metadata provided by user
    pub display_name: Option<String>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "core.auth_provider", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthProvider {
    Email,
    GitHub,
}
