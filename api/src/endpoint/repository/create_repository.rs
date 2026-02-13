use super::default_visibility;
use crate::endpoint::Endpoint;
use crate::resource::RepositoryResource;
use serde::{Deserialize, Serialize};

pub struct CreateRepository;

impl Endpoint for CreateRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}";
    const METHOD: http::Method = http::Method::POST;

    type Request = CreateRepositoryRequest;
    type Response = CreateRepositoryResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRepositoryRequest {
    pub owner_type: String,

    #[serde(default = "default_visibility")]
    pub visibility: String,
}

type CreateRepositoryResponse = RepositoryResource;
