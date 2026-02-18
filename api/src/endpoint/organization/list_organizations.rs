use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::OrganizationResource};

pub struct ListOrganizations;

impl Endpoint for ListOrganizations {
    const PATH: &'static str = "/organizations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationsRequest;
    type Response = ListOrganizationsResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListOrganizationsRequest {}

pub type ListOrganizationsResponse = Vec<OrganizationResource>;
