use chrono::{DateTime, Utc};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::error::{AuthenticationError, InputError};

#[derive(Debug, Clone, FromRow)]
pub struct AccessToken {
    pub id: Uuid,
    pub principal_id: Uuid,
    pub client_id: String,
    pub token_hash: String,
    pub token_type: TokenType,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType {
    Personal,
    Runner,
}

impl TokenType {
    pub fn prefix(&self) -> &'static str {
        match self {
            TokenType::Personal => "gdp_",
            TokenType::Runner => "gdr_",
        }
    }
}

impl TryFrom<&str> for TokenType {
    type Error = AuthenticationError;

    fn try_from(token_type: &str) -> Result<Self, Self::Error> {
        match token_type {
            "personal" => Ok(TokenType::Personal),
            "runner" => Ok(TokenType::Runner),
            _ => Err(
                InputError::new("token_type", format!("Invalid token type: {}", token_type)).into(),
            ),
        }
    }
}

impl Into<String> for TokenType {
    fn into(self) -> String {
        match self {
            TokenType::Personal => "personal".to_string(),
            TokenType::Runner => "runner".to_string(),
        }
    }
}
