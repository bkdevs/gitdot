use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryResource};

pub struct ListUserStars;

impl Endpoint for ListUserStars {
    const PATH: &'static str = "/user/{user_name}/stars";
    const METHOD: http::Method = http::Method::GET;

    type Request = ListUserStarsRequest;
    type Response = ListUserStarsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ListUserStarsRequest {}

pub type ListUserStarsResponse = Vec<RepositoryResource>;
