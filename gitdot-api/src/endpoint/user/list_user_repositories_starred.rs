use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, repository::RepositoryResource},
};

pub struct ListUserStarredRepositories;

impl Endpoint for ListUserStarredRepositories {
    const PATH: &'static str = "/user/{user_name}/repositories-starred";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserStarredRepositoriesRequest;
    type Response = ListUserStarredRepositoriesResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListUserStarredRepositoriesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListUserStarredRepositoriesResponse = Page<RepositoryResource>;
