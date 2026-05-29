use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::RepositoryResource};

pub struct UpdateRepository;

impl Endpoint for UpdateRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}";
    const METHOD: http::Method = http::Method::PATCH;

    type Request = UpdateRepositoryRequest;
    type Response = UpdateRepositoryResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct UpdateRepositoryRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

pub type UpdateRepositoryResponse = RepositoryResource;
