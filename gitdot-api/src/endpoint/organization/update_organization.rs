use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::OrganizationResource};

pub struct UpdateOrganization;

impl Endpoint for UpdateOrganization {
    const PATH: &'static str = "/organization/{org_name}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateOrganizationRequest;
    type Response = UpdateOrganizationResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateOrganizationRequest {
    pub location: Option<String>,
    pub readme: Option<String>,
    pub links: Option<Vec<String>>,
    pub display_name: Option<String>,
}

pub type UpdateOrganizationResponse = OrganizationResource;
