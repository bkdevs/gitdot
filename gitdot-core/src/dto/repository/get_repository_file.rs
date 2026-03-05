#[derive(Debug, Clone)]
pub struct RepositoryFileResponse {
    pub ref_name: String,
    pub path: String,
    pub commit_sha: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}
