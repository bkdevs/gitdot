use serde::{Deserialize, Serialize};

use gitdot_core::dto::RepositoryCommitsResponse;

use super::{RepositoryCommitServerResponse, default_page, default_per_page, default_ref};

#[derive(Deserialize)]
pub struct GetRepositoryCommitsQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetRepositoryCommitsServerResponse {
    pub commits: Vec<RepositoryCommitServerResponse>,
    pub has_next: bool,
}

impl From<RepositoryCommitsResponse> for GetRepositoryCommitsServerResponse {
    fn from(response: RepositoryCommitsResponse) -> Self {
        Self {
            commits: response.commits.into_iter().map(Into::into).collect(),
            has_next: response.has_next,
        }
    }
}
