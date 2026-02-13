use super::default_ref;
use crate::endpoint::Endpoint;
use crate::resource::repository::RepositoryFileResource;
use serde::{Deserialize, Serialize};

pub struct GetRepositoryCommits;

impl Endpoint for GetRepositoryCommits {
    const PATH: &'static str = "/repository/{owner}/{repo}/file";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryFileRequest;
    type Response = GetRepositoryFileResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryFileRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

type GetRepositoryFileResponse = RepositoryFileResource;
