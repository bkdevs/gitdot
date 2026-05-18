use serde::{Deserialize, Serialize};

use crate::{
    endpoint::Endpoint,
    resource::{common::Page, repository::RepositoryResource},
};

pub struct ListUserStars;

impl Endpoint for ListUserStars {
    const PATH: &'static str = "/user/{user_name}/stars";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserStarsRequest;
    type Response = ListUserStarsResponse;
}

#[derive(ApiRequest, Debug, Default, Serialize, Deserialize)]
pub struct ListUserStarsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub type ListUserStarsResponse = Page<RepositoryResource>;
