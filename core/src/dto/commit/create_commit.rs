use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateCommitRequest {
    pub author_id: Uuid,
    pub repo_id: Uuid,
    pub sha: String,
    pub message: String,
}

impl CreateCommitRequest {
    pub fn new(author_id: Uuid, repo_id: Uuid, sha: &str, message: &str) -> Self {
        Self {
            author_id,
            repo_id,
            sha: sha.to_string(),
            message: message.to_string(),
        }
    }
}
