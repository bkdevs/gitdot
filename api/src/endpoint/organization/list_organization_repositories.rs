use serde::{Deserialize, Serialize};

use api_derive::ApiRequest;

use crate::{endpoint::Endpoint, resource::RepositoryResource};

pub struct ListOrganizationRepositories;

impl Endpoint for ListOrganizationRepositories {
    const PATH: &'static str = "/organization/{org_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationRepositoriesRequest;
    type Response = ListOrganizationRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListOrganizationRepositoriesRequest {}

pub type ListOrganizationRepositoriesResponse = Vec<RepositoryResource>;
