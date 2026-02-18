use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::OrganizationMemberResource};

pub struct AddMember;

impl Endpoint for AddMember {
    const PATH: &'static str = "/organization/{org_name}/repositories";
    const METHOD: http::Method = http::Method::POST;

    type Request = AddMemberRequest;
    type Response = AddMemberResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub user_name: String,
    pub role: String,
}

pub type AddMemberResponse = OrganizationMemberResource;
