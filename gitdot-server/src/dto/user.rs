use gitdot_api::resource::{repository as repo_api, user as api};
use gitdot_core::{
    dto::{
        GetCurrentUserResponse, UserCommitResponse, UserEmailResponse, UserOrganizationResponse,
        UserRepositoryResponse, UserResponse,
    },
    model::OrganizationRole,
};

use super::IntoApi;

impl IntoApi for UserResponse {
    type ApiType = api::UserResource;
    fn into_api(self) -> Self::ApiType {
        api::UserResource {
            id: self.id,
            name: self.name,
            location: self.location,
            readme: self.readme,
            links: self.links,
            display_name: self.display_name,
            created_at: self.created_at,
            image_updated_at: self.image_updated_at,
            followers: self.followers,
            following: self.following,
            user_follow: self.user_follow,
        }
    }
}

impl IntoApi for UserEmailResponse {
    type ApiType = api::UserEmailResource;
    fn into_api(self) -> Self::ApiType {
        api::UserEmailResource {
            email: self.email,
            is_primary: self.is_primary,
            is_verified: self.is_verified,
            created_at: self.created_at,
        }
    }
}

impl IntoApi for GetCurrentUserResponse {
    type ApiType = api::CurrentUserResource;
    fn into_api(self) -> Self::ApiType {
        api::CurrentUserResource {
            id: self.id,
            name: self.name,
            emails: self.emails.into_api(),
            memberships: self.memberships.into_api(),
            location: self.location,
            readme: self.readme,
            links: self.links,
            display_name: self.display_name,
            created_at: self.created_at,
            image_updated_at: self.image_updated_at,
            followers: self.followers,
            following: self.following,
            user_follow: self.user_follow,
        }
    }
}

impl IntoApi for UserOrganizationResponse {
    type ApiType = api::UserOrganizationResource;
    fn into_api(self) -> Self::ApiType {
        api::UserOrganizationResource {
            id: self.id,
            name: self.name,
            display_name: self.display_name,
            role: match self.role {
                OrganizationRole::Admin => "admin".to_string(),
                OrganizationRole::Member => "member".to_string(),
            },
            role_description: self.role_description,
            joined_at: self.joined_at,
            image_updated_at: self.image_updated_at,
        }
    }
}

impl IntoApi for UserRepositoryResponse {
    type ApiType = api::UserRepositoryResource;
    fn into_api(self) -> Self::ApiType {
        api::UserRepositoryResource {
            owner: self.owner,
            name: self.name,
            description: self.description,
            stars: self.stars,
            visibility: self.visibility,
            commit_count: self.commit_count,
            last_commit_at: self.last_commit_at,
        }
    }
}

impl IntoApi for UserCommitResponse {
    type ApiType = api::UserCommitResource;
    fn into_api(self) -> Self::ApiType {
        let author = match (self.git_author_name, self.git_author_email, self.author_id) {
            (Some(git_name), Some(_), id) => Some(repo_api::CommitAuthorResource {
                id,
                name: self.author_name,
                git_name,
                image_updated_at: self.author_image_updated_at,
            }),
            _ => None,
        };
        api::UserCommitResource {
            id: self.id,
            date: self.created_at,
            redacted: self.redacted,
            owner_name: self.owner_name,
            repo_name: self.repo_name,
            sha: self.sha,
            parent_sha: self.parent_sha,
            message: self.message,
            author,
            review_number: self.review_number,
            diff_position: self.diff_position,
            diffs: self.diffs.into_iter().map(|d| d.into_api()).collect(),
        }
    }
}
