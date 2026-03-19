use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSettings {
    pub repos: HashMap<String, UserRepoSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRepoSettings {
    pub commit_filters: Option<Vec<CommitFilter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSettings {
    pub commit_filters: Option<Vec<CommitFilter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitFilter {
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub included_paths: Option<Vec<String>>,
    pub excluded_paths: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
