use super::{RepositoryCommitApiResponse, default_page, default_per_page, default_ref};
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct GetRepositoryFileCommits;

impl Endpoint for GetRepositoryFileCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/file/commits";
    const METHOD: http::Method = http::Method::GET;

    type ApiRequest = GetRepositoryFileCommitsApiRequest;
    type ApiResponse = GetRepositoryFileCommitsApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryFileCommitsApiRequest {
    pub path: String,
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryFileCommitsApiResponse {
    pub commits: Vec<RepositoryCommitApiResponse>,
    pub has_next: bool,
}
