use crate::endpoint::Endpoint;
use crate::resource::RepositoryResource;
use serde::{Deserialize, Serialize};

pub struct ListOrganizationRepositories;

impl Endpoint for ListOrganizationRepositories {
    const PATH: &'static str = "/organization/{org_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListOrganizationRepositoriesRequest;
    type Response = ListOrganizationRepositoriesResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListOrganizationRepositoriesRequest {}

pub type ListOrganizationRepositoriesResponse = Vec<RepositoryResource>;
