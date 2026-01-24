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
pub struct GetRepositoryFileServerResponse {}

impl From<RepositoryFileResponse> for GetRepositoryFileServerResponse {
    fn from(response: RepositoryFileResponse) -> Self {
        todo!()
    }
}
