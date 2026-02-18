use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::RepositoryResource};

pub struct ListOrganizationRepositories;

impl Endpoint for ListOrganizationRepositories {
    const PATH: &'static str = "/organization/{org_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationRepositoriesRequest;
    type Response = ListOrganizationRepositoriesResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct ListOrganizationRepositoriesRequest {}

pub type ListOrganizationRepositoriesResponse = Vec<RepositoryResource>;
