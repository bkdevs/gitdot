#[derive(serde::Serialize)]
pub struct RepositoryTree {
    pub ref_name: String,
    pub commit_sha: String,
    pub path: String,
    pub entries: Vec<RepositoryTreeEntry>,
}

#[derive(serde::Serialize)]
pub struct RepositoryTreeEntry {
    pub name: String,
    pub entry_type: String,
    pub sha: String,
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
