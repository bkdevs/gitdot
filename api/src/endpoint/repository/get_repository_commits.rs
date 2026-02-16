use serde::{Deserialize, Serialize};

use super::{default_page, default_per_page, default_ref};
use crate::endpoint::Endpoint;
use crate::resource::repository::RepositoryCommitsResource;

pub struct GetRepositoryCommits;

impl Endpoint for GetRepositoryCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/commits";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryCommitsRequest;
    type Response = GetRepositoryCommitsResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryCommitsRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

pub type GetRepositoryCommitsResponse = RepositoryCommitsResource;
