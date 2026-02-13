use super::{RepositoryCommitApiResponse, default_page, default_per_page, default_ref};
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct GetRepositoryCommits;

impl Endpoint for GetRepositoryCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits";
    const METHOD: http::Method = http::Method::GET;

    type ApiRequest = GetRepositoryCommitsApiRequest;
    type ApiResponse = GetRepositoryCommitsApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitsApiRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitsApiResponse {
    pub commits: Vec<RepositoryCommitApiResponse>,
    pub has_next: bool,
}
