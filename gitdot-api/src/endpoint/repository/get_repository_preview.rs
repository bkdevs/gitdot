use serde::{Deserialize, Serialize};

use super::default_ref;
use crate::{endpoint::Endpoint, resource::repository::RepositoryPreviewResource};

pub struct GetRepositoryPreview;

impl Endpoint for GetRepositoryPreview {
    const PATH: &'static str = "/repository/{owner}/{repo}/preview";
    const METHOD: http::Method = http::Method::GET;

    type Request = GetRepositoryPreviewRequest;
    type Response = GetRepositoryPreviewResponse;
}

#[derive(ApiRequest, Debug, Serialize, Deserialize)]
pub struct GetRepositoryPreviewRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub preview_lines: Option<u32>,
}

pub type GetRepositoryPreviewResponse = RepositoryPreviewResource;
