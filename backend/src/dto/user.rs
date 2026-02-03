mod create_user;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use gitdot_core::dto::UserResponse;

pub use create_user::CreateUserServerRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserServerResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<UserResponse> for UserServerResponse {
    fn from(response: UserResponse) -> Self {
        Self {
            id: response.id,
            name: response.name,
            email: response.email,
            created_at: response.created_at,
        }
    }
}
