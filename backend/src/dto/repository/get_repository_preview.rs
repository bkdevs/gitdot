use serde::{Deserialize, Serialize};

use gitdot_core::dto::{FilePreview, RepositoryPreviewEntry, RepositoryPreviewResponse};

use super::default_ref;

#[derive(Deserialize)]
pub struct GetRepositoryPreviewQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub preview_lines: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetRepositoryPreviewServerResponse {
    pub name: String,
    pub owner: String,
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryPreviewEntryServerResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RepositoryPreviewEntryServerResponse {
    pub path: String,
    pub name: String,
    pub sha: String,
    pub preview: Option<FilePreviewServerResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FilePreviewServerResponse {
    pub content: String,
    pub total_lines: u32,
    pub preview_lines: u32,
    pub truncated: bool,
    pub encoding: String,
}

impl From<RepositoryPreviewResponse> for GetRepositoryPreviewServerResponse {
    fn from(response: RepositoryPreviewResponse) -> Self {
        Self {
            name: response.name,
            owner: response.owner,
            ref_name: response.ref_name,
            commit_sha: response.commit_sha,
            entries: response
                .entries
                .into_iter()
                .filter(|e| e.entry_type == "blob")
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<RepositoryPreviewEntry> for RepositoryPreviewEntryServerResponse {
    fn from(entry: RepositoryPreviewEntry) -> Self {
        Self {
            path: entry.path,
            name: entry.name,
            sha: entry.sha,
            preview: entry.preview.map(Into::into),
        }
    }
}

impl From<FilePreview> for FilePreviewServerResponse {
    fn from(preview: FilePreview) -> Self {
        Self {
            content: preview.content,
            total_lines: preview.total_lines,
            preview_lines: preview.preview_lines,
            truncated: preview.truncated,
            encoding: preview.encoding,
        }
    }
}
