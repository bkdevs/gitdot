use serde::{Deserialize, Serialize};

use gitdot_core::dto::RepositoryTreeResponse;

use super::default_ref;

#[derive(Deserialize)]
pub struct GetRepositoryTreeQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GetRepositoryTreeServerResponse {}

impl From<RepositoryTreeResponse> for GetRepositoryTreeServerResponse {
    fn from(response: RepositoryTreeResponse) -> Self {
        todo!()
    }
}
