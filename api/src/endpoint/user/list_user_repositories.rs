use crate::endpoint::Endpoint;
use crate::endpoint::repository::RepositoryApiResponse;
use serde::{Deserialize, Serialize};

pub struct ListUserRepositories;

impl Endpoint for ListUserRepositories {
    const PATH: &'static str = "/user/{user_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type ApiRequest = ListUserRespositoriesApiRequest;
    type ApiResponse = ListUserRepositoriesApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserRespositoriesApiRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserRepositoriesApiResponse {
    pub repositories: Vec<RepositoryApiResponse>,
}
