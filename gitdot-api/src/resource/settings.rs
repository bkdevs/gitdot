use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(ApiResource, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitFilterResource {
    pub name: String,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub included_paths: Option<Vec<String>>,
    pub excluded_paths: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
