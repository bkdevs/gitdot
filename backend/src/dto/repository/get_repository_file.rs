use serde::{Deserialize, Serialize};

use gitdot_core::dto::RepositoryFileResponse;

use super::default_ref;

#[derive(Deserialize)]
pub struct GetRepositoryFileQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetRepositoryFileServerResponse {
    name: String,
    owner: String,
    ref_name: String,
    path: String,
    commit_sha: String,
    sha: String,
    content: String,
    encoding: String,
}

impl From<RepositoryFileResponse> for GetRepositoryFileServerResponse {
    fn from(response: RepositoryFileResponse) -> Self {
        Self {
            name: response.name,
            owner: response.owner,
            ref_name: response.ref_name,
            path: response.path,
            commit_sha: response.commit_sha,
            sha: response.sha,
            content: response.content,
            encoding: response.encoding,
        }
    }
}
