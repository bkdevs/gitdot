#[derive(serde::Deserialize)]
pub struct CreateRepositoryRequest {
    #[serde(default = "default_branch")]
    pub default_branch: String,
}

#[derive(serde::Serialize)]
pub struct CreateRepositoryResponse {
    pub owner: String,
    pub name: String,
    pub default_branch: String,
}

#[derive(serde::Deserialize)]
pub struct RepositoryTreeQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    #[serde(default)]
    pub path: String,
}

#[derive(serde::Serialize)]
pub struct RepositoryTree {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub entries: Vec<RepositoryTreeEntry>,
}

#[derive(serde::Serialize)]
pub struct RepositoryTreeEntry {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
}

#[derive(serde::Deserialize)]
pub struct RepositoryFileQuery {
    #[serde(default = "default_ref")]
    pub ref_name: String,
    pub path: String,
}

#[derive(serde::Serialize)]
pub struct RepositoryFile {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}

fn default_branch() -> String {
    "main".to_string()
}

fn default_ref() -> String {
    "HEAD".to_string()
}
