use super::default_ref;
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct GetRepositoryCommits;

impl Endpoint for GetRepositoryCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/file";
    const METHOD: http::Method = http::Method::GET;

    type ApiRequest = GetRepositoryFileApiRequest;
    type ApiResponse = GetRepositoryFileApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryFileApiRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryFileApiResponse {
    name: String,
    owner: String,
    ref_name: String,
    path: String,
    commit_sha: String,
    sha: String,
    content: String,
    encoding: String,
}
