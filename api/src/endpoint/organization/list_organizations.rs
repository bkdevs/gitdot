use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::OrganizationResource};

pub struct ListOrganizations;

impl Endpoint for ListOrganizations {
    const PATH: &'static str = "/organizations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationsRequest;
    type Response = ListOrganizationsResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct ListOrganizationsRequest {}

pub type ListOrganizationsResponse = Vec<OrganizationResource>;
