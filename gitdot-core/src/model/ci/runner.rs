use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::error::{InputError, RunnerError};

#[derive(Debug, Clone, FromRow)]
pub struct Runner {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub owner_name: String,
    pub owner_type: RunnerOwnerType,
    pub last_active: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "runner_owner_type", rename_all = "lowercase")]
pub enum RunnerOwnerType {
    User,
    Organization,
}

impl TryFrom<&str> for RunnerOwnerType {
    type Error = RunnerError;

    fn try_from(owner_type: &str) -> Result<Self, Self::Error> {
        match owner_type {
            "user" => Ok(RunnerOwnerType::User),
            "organization" => Ok(RunnerOwnerType::Organization),
            _ => Err(InputError::new("owner type", owner_type).into()),
        }
    }
}

impl Into<String> for RunnerOwnerType {
    fn into(self) -> String {
        match self {
            RunnerOwnerType::User => "user".to_string(),
            RunnerOwnerType::Organization => "organization".to_string(),
        }
    }
}
