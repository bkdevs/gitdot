use serde::{Deserialize, Serialize};

use api_derive::ApiRequest;

use crate::{endpoint::Endpoint, resource::OrganizationResource};

pub struct ListUserOrganizations;

impl Endpoint for ListUserOrganizations {
    const PATH: &'static str = "/user/{user_name}/organizations";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserOrganizationsRequest;
    type Response = ListUserOrganizationsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListUserOrganizationsRequest {}

pub type ListUserOrganizationsResponse = Vec<OrganizationResource>;
