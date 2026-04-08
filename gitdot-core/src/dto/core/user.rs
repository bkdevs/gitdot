mod get_current_user;
mod get_current_user_settings;
mod get_user;
mod has_user;
mod list_user_commits;
mod list_user_organizations;
mod list_user_repositories;
mod list_user_reviews;
mod update_current_user;
mod update_current_user_settings;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use std::collections::HashMap;

use crate::model::{CommitFilter, User, UserRepoSettings, UserSettings};

pub use get_current_user::GetCurrentUserRequest;
pub use get_current_user_settings::GetCurrentUserSettingsRequest;
pub use get_user::GetUserRequest;
pub use has_user::HasUserRequest;
pub use list_user_commits::ListUserCommitsRequest;
pub use list_user_organizations::ListUserOrganizationsRequest;
pub use list_user_repositories::ListUserRepositoriesRequest;
pub use list_user_reviews::ListUserReviewsRequest;
pub use update_current_user::UpdateCurrentUserRequest;
pub use update_current_user_settings::UpdateCurrentUserSettingsRequest;

#[derive(Debug, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
    pub company: Option<String>,
    pub image: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            location: user.location,
            readme: user.readme,
            links: user.links,
            company: user.company,
            image: user.image,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserSettingsResponse {
    pub repos: HashMap<String, UserRepoSettingsResponse>,
}

impl From<UserSettings> for UserSettingsResponse {
    fn from(s: UserSettings) -> Self {
        Self {
            repos: s.repos.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRepoSettingsResponse {
    pub commit_filters: Option<Vec<CommitFilter>>,
}

impl From<UserRepoSettings> for UserRepoSettingsResponse {
    fn from(s: UserRepoSettings) -> Self {
        Self {
            commit_filters: s.commit_filters,
        }
    }
}
