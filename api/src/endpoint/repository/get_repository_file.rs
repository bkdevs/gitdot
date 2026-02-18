use api_derive::ApiRequest;
use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryFileResource};

pub struct GetRepositoryFile;

impl Endpoint for GetRepositoryFile {
    const PATH: &'static str = "/repository/{owner}/{repo}/file";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryFileRequest;
    type Response = GetRepositoryFileResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryFileRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

pub type GetRepositoryFileResponse = RepositoryFileResource;
