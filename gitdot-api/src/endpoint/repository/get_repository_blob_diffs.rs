use serde::{Deserialize, Serialize};

use crate::{endpoint::Endpoint, resource::repository::RepositoryBlobDiffsResource};

pub struct GetRepositoryBlobDiffs;

impl Endpoint for GetRepositoryBlobDiffs {
    const PATH: &'static str = "/repository/{owner}/{repo}/blob/diffs";
    const METHOD: http::Method = http::Method::POST;

    type Request = GetRepositoryBlobDiffsRequest;
    type Response = GetRepositoryBlobDiffsResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryBlobDiffsRequest {
    pub commit_shas: Vec<String>,
    pub path: String,
}

pub type GetRepositoryBlobDiffsResponse = RepositoryBlobDiffsResource;
