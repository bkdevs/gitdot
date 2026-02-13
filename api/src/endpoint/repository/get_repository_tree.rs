use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryTreeResource};
use serde::{Deserialize, Serialize};

pub struct GetRepositoryTree;

impl Endpoint for GetRepositoryTree {
    const PATH: &'static str = "/repository/{owner}/{repo}/tree";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryTreeRequest;
    type Response = GetRepositoryTreeResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryTreeRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
}

pub type GetRepositoryTreeResponse = RepositoryTreeResource;
