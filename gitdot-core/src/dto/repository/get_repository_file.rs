#[derive(Debug, Clone)]
pub struct RepositoryFileResponse {
    pub commit_sha: String,
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}
