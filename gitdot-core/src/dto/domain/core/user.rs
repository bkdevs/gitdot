mod follow_user;
mod get_current_user;
mod get_user;
mod has_user;
mod list_user_commits;
mod list_user_organizations;
mod list_user_repositories;
mod list_user_repositories_contributed;
mod list_user_repositories_starred;
mod list_user_reviews;
mod unfollow_user;
mod update_current_user;
mod update_current_user_image;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{OrganizationRole, Repository, User, UserEmail, UserOrganization};

pub use follow_user::FollowUserRequest;
pub use get_current_user::GetCurrentUserRequest;
pub use get_user::GetUserRequest;
pub use has_user::HasUserRequest;
pub use list_user_commits::ListUserCommitsRequest;
pub use list_user_organizations::ListUserOrganizationsRequest;
pub use list_user_repositories::ListUserRepositoriesRequest;
pub use list_user_repositories_contributed::ListUserContributedRepositoriesRequest;
pub use list_user_repositories_starred::ListUserStarredRepositoriesRequest;
pub use list_user_reviews::ListUserReviewsRequest;
pub use unfollow_user::UnfollowUserRequest;
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
    pub image_updated_at: DateTime<Utc>,
    pub followers: i64,
    pub following: i64,
    pub user_follow: bool,
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
            image_updated_at: user.image_updated_at,
            followers: user.followers,
            following: user.following,
            user_follow: user.user_follow,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRepositoryResponse {
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub stars: u32,
    pub visibility: String,

    pub commit_count: Option<u32>,
    pub last_commit_at: Option<DateTime<Utc>>,
}

impl UserRepositoryResponse {
    pub fn from_repository(
        repo: Repository,
        commit_count: Option<i64>,
        last_commit_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            owner: repo.owner_name,
            name: repo.name,
            description: repo.description,
            stars: repo.stars as u32,
            visibility: repo.visibility.into(),
            commit_count: commit_count.map(|c| c as u32),
            last_commit_at,
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
    pub memberships: Vec<UserOrganizationResponse>,

    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Vec<String>,
    pub display_name: Option<String>,

    pub created_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,
    pub followers: i64,
    pub following: i64,
    pub user_follow: bool,
}

#[derive(Debug, Clone)]
pub struct UserOrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub display_name: Option<String>,

    pub role: OrganizationRole,
    pub role_description: Option<String>,
    pub joined_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,
}

impl From<UserOrganization> for UserOrganizationResponse {
    fn from(org: UserOrganization) -> Self {
        Self {
            id: org.id,
            name: org.name,
            display_name: org.display_name,
            role: org.role,
            role_description: org.role_description,
            joined_at: org.joined_at,
            image_updated_at: org.image_updated_at,
        }
    }
}
