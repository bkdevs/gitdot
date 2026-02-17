use serde::{Deserialize, Serialize};

use crate::endpoint::Endpoint;

pub struct DeleteRepository;

impl Endpoint for DeleteRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}";
    const METHOD: http::Method = http::Method::DELETE;

    type Request = DeleteRepositoryRequest;
    type Response = DeleteRepositoryResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRepositoryRequest {}

pub type DeleteRepositoryResponse = ();
