mod add_user_email;
mod get_current_user;
mod get_user;
mod has_user;
mod list_user_commits;
mod list_user_organizations;
mod list_user_repositories;
mod list_user_reviews;
mod list_user_stars;
mod update_current_user;
mod update_current_user_image;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    dto::OrganizationMemberResponse,
    model::{User, UserEmail},
};

pub use add_user_email::AddUserEmailRequest;
pub use get_current_user::GetCurrentUserRequest;
pub use get_user::GetUserRequest;
pub use has_user::HasUserRequest;
pub use list_user_commits::ListUserCommitsRequest;
pub use list_user_organizations::ListUserOrganizationsRequest;
pub use list_user_repositories::ListUserRepositoriesRequest;
pub use list_user_reviews::ListUserReviewsRequest;
pub use list_user_stars::ListUserStarsRequest;
pub use update_current_user::UpdateCurrentUserRequest;
pub use update_current_user_image::UpdateCurrentUserImageRequest;

#[derive(Debug, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,

    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
    pub display_name: Option<String>,

    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            location: user.location,
            readme: user.readme,
            links: user.links,
            display_name: user.display_name,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserEmailResponse {
    pub email: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl From<UserEmail> for UserEmailResponse {
    fn from(e: UserEmail) -> Self {
        Self {
            email: e.email,
            is_primary: e.is_primary,
            is_verified: e.is_verified,
            created_at: e.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GetCurrentUserResponse {
    pub id: Uuid,
    pub name: String,
    pub emails: Vec<UserEmailResponse>,
    pub memberships: Vec<OrganizationMemberResponse>,

    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
    pub display_name: Option<String>,

    pub created_at: DateTime<Utc>,
}
