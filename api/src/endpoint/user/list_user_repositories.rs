use crate::endpoint::{Endpoint, EndpointRequest, EndpointResponse};
use crate::endpoint::repository::RepositoryEndpointResponse;
use serde::{Deserialize, Serialize};

pub struct ListUserRepositories;

impl Endpoint for ListUserRepositories {
    const PATH: &'static str = "/user/{user_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserRepositoriesEndpointRequest;
    type Response = ListUserRepositoriesEndpointResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserRepositoriesEndpointRequest {}

impl EndpointRequest for ListUserRepositoriesEndpointRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserRepositoriesEndpointResponse {
    pub repositories: Vec<RepositoryEndpointResponse>,
}

impl EndpointResponse for ListUserRepositoriesEndpointResponse {}
