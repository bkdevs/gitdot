use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryResource};

pub struct ConvertReadonlyRepository;

impl Endpoint for ConvertReadonlyRepository {
    const PATH: &'static str = "/repository/{owner}/{repo}/convert-readonly";
    const METHOD: http::Method = http::Method::POST;

    type Request = ConvertReadonlyRepositoryRequest;
    type Response = ConvertReadonlyRepositoryResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct ConvertReadonlyRepositoryRequest {}

pub type ConvertReadonlyRepositoryResponse = RepositoryResource;
