use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::resource::repository::{CommitAuthorResource, CommitDiffResource};

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserResource {
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

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserEmailResource {
    pub email: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentUserResource {
    pub id: Uuid,
    pub name: String,
    pub emails: Vec<UserEmailResource>,
    pub memberships: Vec<UserOrganizationResource>,

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

/// An organization as surfaced from a user's perspective. It contains basic org
/// info plus the viewer's own membership metadata (`role`, `joined_at`).
#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOrganizationResource {
    pub id: Uuid,
    pub name: String,
    pub display_name: Option<String>,

    pub role: String,
    pub role_description: Option<String>,
    pub joined_at: DateTime<Utc>,
    pub image_updated_at: DateTime<Utc>,
}

/// A repository as surfaced on a user's profile. It contains the viewer-visible
/// subset of repo fields plus the user's contribution stats.
#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRepositoryResource {
    pub owner: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub stars: u32,
    pub visibility: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_commit_at: Option<DateTime<Utc>>,
}

/// A commit as surfaced on a user's profile. Most fields are optional so that
/// commits in private repositories the viewer cannot access can be returned as
/// a redacted stub (timestamp + `redacted: true` only)
#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCommitResource {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub redacted: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<CommitAuthorResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_number: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_position: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diffs: Vec<CommitDiffResource>,
}
