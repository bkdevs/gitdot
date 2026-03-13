#[derive(Debug, Clone)]
pub struct RepositoryFileResponse {
    pub path: String,
    pub sha: String,
    pub content: String,
    pub encoding: String,
}
