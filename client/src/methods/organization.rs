use anyhow::Result;

use gitdot_api::endpoint::organization::{
    add_member::{AddMemberRequest, AddMemberResponse},
    create_organization::{CreateOrganizationRequest, CreateOrganizationResponse},
    list_organization_repositories::{
        ListOrganizationRepositoriesRequest, ListOrganizationRepositoriesResponse,
    },
    list_organizations::{ListOrganizationsRequest, ListOrganizationsResponse},
};

use crate::client::GitdotClient;

impl GitdotClient {
    pub async fn create_organization(
        &self,
        org_name: &str,
        request: CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse> {
        self.post(format!("organization/{}", org_name), request)
            .await
    }

    pub async fn list_organizations(
        &self,
        request: ListOrganizationsRequest,
    ) -> Result<ListOrganizationsResponse> {
        self.get("organizations".to_string(), request).await
    }

    pub async fn list_organization_repositories(
        &self,
        org_name: &str,
        request: ListOrganizationRepositoriesRequest,
    ) -> Result<ListOrganizationRepositoriesResponse> {
        self.get(format!("organization/{}/repositories", org_name), request)
            .await
    }

    pub async fn add_member(
        &self,
        org_name: &str,
        request: AddMemberRequest,
    ) -> Result<AddMemberResponse> {
        self.post(format!("organization/{}/repositories", org_name), request)
            .await
    }
}
