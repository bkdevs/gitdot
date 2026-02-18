use serde::{Deserialize, Serialize};
use api_derive::EndpointRequest;

use crate::{endpoint::Endpoint, resource::repository::RepositoryResource};

pub struct ListUserRepositories;

impl Endpoint for ListUserRepositories {
    const PATH: &'static str = "/user/{user_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserRepositoriesRequest;
    type Response = ListUserRepositoriesResponse;
}

#[derive(EndpointRequest, Debug, Serialize, Deserialize)]
pub struct ListUserRepositoriesRequest {}

pub type ListUserRepositoriesResponse = Vec<RepositoryResource>;
