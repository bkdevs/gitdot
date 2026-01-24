use serde::{Deserialize, Serialize};

use gitdot_core::dto::{RepositoryTreeEntry, RepositoryTreeResponse};

use super::{RepositoryCommitServerResponse, default_ref};

#[derive(Deserialize)]
pub struct GetRepositoryTreeQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetRepositoryTreeServerResponse {
    pub name: String,
    pub owner: String,
    pub ref_name: String,
    pub commit_sha: String,
    pub entries: Vec<RepositoryTreeEntryServerResponse>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RepositoryTreeEntryServerResponse {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    pub commit: RepositoryCommitServerResponse,
}

impl From<RepositoryTreeResponse> for GetRepositoryTreeServerResponse {
    fn from(response: RepositoryTreeResponse) -> Self {
        Self {
            name: response.name,
            owner: response.owner,
            ref_name: response.ref_name,
            commit_sha: response.commit_sha,
            entries: response.entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<RepositoryTreeEntry> for RepositoryTreeEntryServerResponse {
    fn from(entry: RepositoryTreeEntry) -> Self {
        Self {
            path: entry.path,
            name: entry.name,
            entry_type: entry.entry_type,
            sha: entry.sha,
            commit: entry.commit.into(),
        }
    }
}
