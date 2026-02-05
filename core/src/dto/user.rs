mod get_current_user;
mod get_user;
mod list_user_repositories;
mod validate_name;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::User;

pub use get_current_user::GetCurrentUserRequest;
pub use get_user::GetUserRequest;
pub use list_user_repositories::ListUserRepositoriesRequest;
pub use validate_name::ValidateNameRequest;

#[derive(Debug, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
