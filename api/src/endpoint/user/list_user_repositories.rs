use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryResource};

pub struct ListUserRepositories;

impl Endpoint for ListUserRepositories {
    const PATH: &'static str = "/user/{user_name}/repositories";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserRepositoriesRequest;
    type Response = ListUserRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListUserRepositoriesRequest {}

pub type ListUserRepositoriesResponse = Vec<RepositoryResource>;
