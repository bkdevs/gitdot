use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryActivityEventResource};

pub struct GetRepositoryActivity;

impl Endpoint for GetRepositoryActivity {
    const PATH: &'static str = "/repository/{owner}/{repo}/activity";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryActivityRequest;
    type Response = GetRepositoryActivityResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryActivityRequest {}

pub type GetRepositoryActivityResponse = Vec<RepositoryActivityEventResource>;
