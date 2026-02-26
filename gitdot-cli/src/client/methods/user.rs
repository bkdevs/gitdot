use anyhow::Result;

use gitdot_api::endpoint::user::{
    get_current_user::{GetCurrentUserRequest, GetCurrentUserResponse},
    get_user::{GetUserRequest, GetUserResponse},
    has_user::HasUserApiRequest,
    list_user_organizations::{ListUserOrganizationsRequest, ListUserOrganizationsResponse},
    list_user_repositories::{ListUserRepositoriesRequest, ListUserRepositoriesResponse},
    update_current_user::{UpdateCurrentUserRequest, UpdateCurrentUserResponse},
};

use crate::client::GitdotClient;

#[allow(dead_code)]
impl GitdotClient {
    pub async fn get_current_user(
        &self,
        request: GetCurrentUserRequest,
    ) -> Result<GetCurrentUserResponse> {
        self.get("user".to_string(), request).await
    }

    pub async fn get_user(
        &self,
        user_name: &str,
        request: GetUserRequest,
    ) -> Result<GetUserResponse> {
        self.get(format!("user/{}", user_name), request).await
    }

    pub async fn has_user(&self, user_name: &str, request: HasUserApiRequest) -> Result<()> {
        self.head(format!("user/{}", user_name), request).await
    }

    pub async fn list_user_repositories(
        &self,
        user_name: &str,
        request: ListUserRepositoriesRequest,
    ) -> Result<ListUserRepositoriesResponse> {
        self.get(format!("user/{}/repositories", user_name), request)
            .await
    }

    pub async fn list_user_organizations(
        &self,
        user_name: &str,
        request: ListUserOrganizationsRequest,
    ) -> Result<ListUserOrganizationsResponse> {
        self.get(format!("user/{}/organizations", user_name), request)
            .await
    }

    pub async fn update_current_user(
        &self,
        request: UpdateCurrentUserRequest,
    ) -> Result<UpdateCurrentUserResponse> {
        self.patch("user".to_string(), request).await
    }
}
