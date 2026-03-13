use crate::dto::RepositoryCommitResponse;

#[derive(Debug, Clone)]
pub struct RepositoryTreeEntry {
    pub path: String,
    pub name: String,
    pub entry_type: String,
    pub sha: String,
    pub commit: Option<RepositoryCommitResponse>,
}
