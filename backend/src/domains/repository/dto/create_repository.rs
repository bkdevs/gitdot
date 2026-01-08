use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRepositoryRequest {
    #[serde(default = "default_branch")]
    pub default_branch: String,
}

#[derive(Serialize)]
pub struct CreateRepositoryResponse {
    pub owner: String,
    pub name: String,
    pub default_branch: String,
}

fn default_branch() -> String {
    "main".to_string()
}
