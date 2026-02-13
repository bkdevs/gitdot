use super::{default_page, default_per_page, default_ref};
use crate::{endpoint::Endpoint, resource::repository::RepositoryCommitsResource};
use serde::{Deserialize, Serialize};

pub struct GetRepositoryFileCommits;

impl Endpoint for GetRepositoryFileCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/file/commits";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryFileCommitsRequest;
    type Response = GetRepositoryFileCommitsResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryFileCommitsRequest {
    pub path: String,
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

pub type GetRepositoryFileCommitsResponse = RepositoryCommitsResource;
