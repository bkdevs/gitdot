use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateRepositoryServerRequest {
    pub owner_type: String,

    #[serde(default = "default_visibility")]
    pub visibility: String,
}

fn default_visibility() -> String {
    "public".to_string()
}
