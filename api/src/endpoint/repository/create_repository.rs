use super::{RepositoryApiResponse, default_visibility};
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct CreateRepository;

impl Endpoint for CreateRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}";
    const METHOD: http::Method = http::Method::POST;

    type ApiRequest = CreateRepositoryApiRequest;
    type ApiResponse = RepositoryApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepositoryApiRequest {
    pub owner_type: String,

    #[serde(default = "default_visibility")]
    pub visibility: String,
}
