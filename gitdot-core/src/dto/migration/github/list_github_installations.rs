use uuid::Uuid;

use super::create_github_installation::GitHubInstallationResponse;

#[derive(Debug, Clone)]
pub struct ListGitHubInstallationsRequest {
    pub owner_id: Uuid,
}

impl ListGitHubInstallationsRequest {
    pub fn new(owner_id: Uuid) -> Self {
        Self { owner_id }
    }
}

pub type ListGitHubInstallationsResponse = Vec<GitHubInstallationResponse>;
