use serde::Deserialize;

use super::{default_page, default_per_page, default_ref};

#[derive(Deserialize)]
pub struct GetRepositoryFileCommitsQuery {
    pub path: String,
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}
