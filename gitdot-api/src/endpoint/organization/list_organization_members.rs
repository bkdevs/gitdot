use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::OrganizationMemberResource};

pub struct ListOrganizationMembers;

impl Endpoint for ListOrganizationMembers {
    const PATH: &'static str = "/organization/{org_name}/members";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationMembersRequest;
    type Response = ListOrganizationMembersResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListOrganizationMembersRequest {
    pub role: Option<String>,
}

pub type ListOrganizationMembersResponse = Vec<OrganizationMemberResource>;
