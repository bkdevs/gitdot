use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryResource};

pub struct GetRepository;

impl Endpoint for GetRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryRequest;
    type Response = GetRepositoryResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryRequest {}

pub type GetRepositoryResponse = RepositoryResource;
