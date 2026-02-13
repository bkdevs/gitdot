use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryPreviewResource};
use serde::{Deserialize, Serialize};

pub struct GetRepositoryPreview;

impl Endpoint for GetRepositoryPreview {
    const PATH: &'static str = "/repository/{owner}/{repo}/preview";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryPreviewRequest;
    type Response = GetRepositoryPreviewResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryPreviewRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub preview_lines: Option<u32>,
}

type GetRepositoryPreviewResponse = RepositoryPreviewResource;
