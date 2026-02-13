use super::default_ref;
use crate::endpoint::Endpoint;
use serde::{Deserialize, Serialize};

pub struct GetRepositoryPreview;

impl Endpoint for GetRepositoryPreview {
    const PATH: &'static str = "/repository/{owner}/{repo}/preview";
    const METHOD: http::Method = http::Method::GET;

    type ApiRequest = GetRepositoryPreviewApiRequest;
    type ApiResponse = GetRepositoryPreviewApiResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryPreviewApiRequest {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub preview_lines: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRepositoryPreviewApiResponse {
    pub name: String,
    pub owner: String,
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPreviewEntryApiResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryPreviewEntryApiResponse {
    pub path: String,
    pub name: String,
    pub sha: String,
    pub preview: Option<FilePreviewApiResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePreviewApiResponse {
    pub content: String,
    pub total_lines: u32,
    pub preview_lines: u32,
    pub truncated: bool,
    pub encoding: String,
}
